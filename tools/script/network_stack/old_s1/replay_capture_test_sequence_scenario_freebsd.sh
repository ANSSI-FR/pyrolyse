#!/bin/bash

set -eu
set -o pipefail

echo "replay_capture_test_sequence_scenario: start"

network_interface=$1
test_index_offset=$2
target_directory_path=$3
input_directory=$4
nb_processes=$5
ip_version=$6

if [[ "${ip_version}" == "4" ]]; then
    tcpdump_filter_global='ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[7]>0 or ip[6] & 63>0)))'
    tcpdump_filter_single='host ${TARGET_IPV4_ADDR} and \(\(\(icmp[icmptype] = icmp-echo or icmp[icmptype] = icmp-echoreply\) and icmp[4:2] = $((test_index_offset+test_index))\) or \(ip[20] != 0 and ip[20] != 8 and ip[4:2] = $((test_index_offset+test_index))\)\)'
else 
    tcpdump_filter_global='icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))'
    tcpdump_filter_single='host ${TARGET_IPV6_ADDR} and \(\(ip6 proto 44 and ip6[44:4] = $((test_index_offset+test_index))\) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)'
fi

echo "replay_capture_test_sequence_scenario: network_interface: ${network_interface}"
echo "replay_capture_test_sequence_scenario: tcpdump_filter_global: ${tcpdump_filter_global}"
echo "replay_capture_test_sequence_scenario: tcpdump_filter_single: ${tcpdump_filter_single}"
echo "replay_capture_test_sequence_scenario: test_index_offset: ${test_index_offset}"
echo "replay_capture_test_sequence_scenario: target_directory_path: ${target_directory_path}"
echo "replay_capture_test_sequence_scenario: input_directory: ${input_directory}"
echo "replay_capture_test_sequence_scenario: nb_processes: ${nb_processes}"

printf -v date '%(%Y%m%d_%H%M%S)T' -1

test_case_path="${target_directory_path}/${input_directory}"
echo "replay_capture_test_sequence_scenario: test_case_path: '${test_case_path}'"

ip_version_s=$(echo "${input_directory}" | cut -d'_' -f2)
scenario_s=$(echo "${input_directory}" | cut -d'_' -f3)

output_pcap_directory_latest="${target_directory_path}/output/${ip_version_s}_${scenario_s}_latest"
output_pcap_directory_current="${target_directory_path}/output/${ip_version_s}_${scenario_s}_${date}"
mkdir -p "${output_pcap_directory_current}"
mkdir -p "${output_pcap_directory_latest}"

# We extract the (original) name from the (potential) altname because tcpreplay does not support interface altnames.
network_interface_name=$(ip link show dev "${network_interface}" | head -n 1 | cut -d':' -f2 | xargs)
echo "replay_capture_test_sequence_scenario: network_interface_name: '${network_interface_name}'"

export network_interface_name
export tcpdump_filter_global
export tcpdump_filter_single
export test_index_offset
export target_directory_path
export output_pcap_directory_current

export test_case_path

# TODO: rewrite this by launching a single tcpdump that writes a single pcap and splitting it later to minimize hard-drive write

function replay_capture_single_pcap() {
    set -e

    echo ""
    echo ""
    echo ""
    echo "replay_capture_single_pcap: start"
    pcap_name=$1
    pcap_path="${test_case_path}/${pcap_name}"
    pcap_path_wo_ext="${pcap_path%.*}"
    echo "replay_capture_single_pcap: pcap_path: ${pcap_path}"
    echo "replay_capture_single_pcap: pcap_path_wo_ext: ${pcap_path_wo_ext}"
    
    test_index=$(basename "${pcap_path_wo_ext}" | cut -d'_' -f2)
    echo "replay_capture_single_pcap: test_index: ${test_index}"
    pcap_test_path="${output_pcap_directory_current}/output_${test_index}.pcap"
    echo "replay_capture_single_pcap: pcap_test_path: ${pcap_test_path}"
    
    # TODO: check security of using eval here (check https://stackoverflow.com/questions/17529220/why-should-eval-be-avoided-in-bash-and-what-should-i-use-instead)
    # We expand the variable inside the BPF filter. We can thus use test_index inside the filter.
    set -x
    tcpdump_filter_single_expanded=$(eval echo "${tcpdump_filter_single}")
    # tcpdump_filter_single_expanded=${tcpdump_filter_single}
    # tcpdump_filter_single_expanded=$(echo "${tcpdump_filter_single}" | sed "s/${test_index}/${test_index}/g")
    set +x
    echo "replay_capture_single_pcap: tcpdump_filter_single_expanded: ${tcpdump_filter_single_expanded}"

    # NB: next line is and old note when filter where defined in this script and not provided as argument.
    # We filter TTL exceeded message because they sometimes appear in traces.

    # NB: sending SIGTERM to tcpdump allows it to flush/save packets to the file.
    # sending SIGKILL to tcpdump may leave unflushed packets unsaved to the file.
    # 
    # sudo tcpdump -i "${network_interface_name}" 'icmp' -w "${pcap_test_path}" &
    # sleep 10
    # https://superuser.com/questions/297521/how-to-capture-last-n-seconds-of-packets-using-tcpdump
    echo "replay_capture_single_pcap: launching answering traffic capture using tcpdump"
    set -x
    # NB: this command is never working because nb_tcpdump_processes is always 1 even if we use "sleep 10" after tcpreplay.
    # Source: https://stackoverflow.com/questions/25731643/how-to-schedule-tcpdump-to-run-for-a-specific-period-of-time
    # sudo tcpdump -G 5 -W 1 -i "${network_interface_name}" "${tcpdump_filter_single_expanded}" -w "${pcap_test_path}" &
    sudo timeout -s SIGTERM 5 \
    tcpdump -i "${network_interface_name}" "${tcpdump_filter_single_expanded}" -w "${pcap_test_path}" &
    sudo_tcpdump_pid_single=$!
    set +x
    echo "replay_capture_single_pcap: sudo_tcpdump_pid_single: ${sudo_tcpdump_pid_single}"
    
    sleep 2
    echo "replay_capture_single_pcap: replaying trace with tcpreplay"
    # sudo tcpreplay -i "${network_interface_name}" -t "${pcap_path}"
    sudo tcpreplay -i "${network_interface_name}" --pps=10 "${pcap_path}"

    sleep 5

    # nb_tcpdump_processes=$(ps aux | grep tcpdump | grep "${sudo_tcpdump_pid_single}" | wc -l)
    nb_tcpdump_processes=$(ps u --pid "${sudo_tcpdump_pid_single}" | tail +2 | wc -l)
    echo "replay_capture_single_pcap: nb_tcpdump_processes before kill -15 -P on tcpdump: ${nb_tcpdump_processes}"
    # TODO: only send signal if nb_tcpdump_processes > 0 ?

    # We allow error here because if we do not, the return code of tcpdump (124) causes a failure.
    set +e
    # Kill tcpdump (process with sudo's PID as parent)
    sudo pkill -15 -P "${sudo_tcpdump_pid_single}"

    # We sleep 2s to allow the signal to have effect. If not, the next "ps [...]" is often not empty.
    sleep 2
    set -e

    # nb_processes=$(ps aux | grep tcpdump | grep "${sudo_tcpdump_pid_single}" | wc -l)
    # ps u --pid "${sudo_tcpdump_pid_single}"
    nb_tcpdump_processes=$(ps u --pid "${sudo_tcpdump_pid_single}" | tail +2 | wc -l)
    echo "replay_capture_single_pcap: nb_tcpdump_processes after kill -15 -P on tcpdump: ${nb_tcpdump_processes}"
    if [ ${nb_tcpdump_processes} -gt 0 ]; then
        echo "replay_capture_single_pcap: still some processes with PID=${sudo_tcpdump_pid_single} after SIGTERM sent => error"
        # ps u --pid "${sudo_tcpdump_pid_single}"
        exit 1
    fi

    # The wait command returns the return code of waited process. 
    # It here is supposed to be 124 because we sent SIGTERM to tcpdump.
    # We thus fail if return code is not 124.
    set +e
    wait "${sudo_tcpdump_pid_single}"
    tcpdump_return_code=$?
    echo "replay_capture_single_pcap: tcpdump_return_code: ${tcpdump_return_code}"
    set -e
    if [ ${tcpdump_return_code} -ne 124 ]; then
        echo "replay_capture_single_pcap: tcpdump_return_code (${tcpdump_return_code}) != 124 (expected value for SIGTERM) !!!!"
        exit 1
    fi

    # Kill sudo
    # nb_tcpdump_processes=$(ps aux | grep tcpdump | grep "${sudo_tcpdump_pid_single}" | wc -l)
    # nb_tcpdump_processes=$(ps u --pid "${sudo_tcpdump_pid_single}" | tail +2 | wc -l)
    # echo "replay_capture_single_pcap: nb_tcpdump_processes before kill -9 on tcpdump: ${nb_tcpdump_processes}"
    # sudo kill -9 "${sudo_tcpdump_pid_single}"
    # nb_tcpdump_processes=$(ps aux | grep tcpdump | grep "${sudo_tcpdump_pid_single}" | wc -l)
    # nb_tcpdump_processes=$(ps u --pid "${sudo_tcpdump_pid_single}" | tail +2 | wc -l)
    # echo "replay_capture_single_pcap: nb_tcpdump_processes after kill -9 on tcpdump: ${nb_tcpdump_processes}"
    
    echo "replay_capture_single_pcap: end"
}
export -f replay_capture_single_pcap


echo "replay_capture_test_sequence_scenario: cleaning old tcpdump and tcpreplay processes"
set +e
sudo pkill tcpdump
sudo pkill tcpreplay
set -e

pcap_all_path_latest="${target_directory_path}/output/${ip_version_s}_${scenario_s}_all_latest.pcap"
pcap_all_path="${target_directory_path}/output/${ip_version_s}_${scenario_s}_all_${date}.pcap"

#set -x
#tcpdump_filter_global_expanded=$(eval echo "${tcpdump_filter_global}")
#set +x
#echo "replay_capture_test_sequence_scenario: tcpdump_filter_global_expanded: ${tcpdump_filter_global_expanded}"
sudo tcpdump -i "${network_interface_name}" "${tcpdump_filter_global}" -w "${pcap_all_path}" &
#sudo tcpdump -i "${network_interface_name}" "${tcpdump_filter_global}" -w "${pcap_all_path}" &
sudo_tcpdump_pid_global=$!
    
# find "${pcap_path}" -name "*.pcap"
find "${test_case_path}" -name "*.pcap" > pcap_files_log
nb_pcap_file=$(find "${test_case_path}" -name "*.pcap" | wc -l)
echo "replay_capture_test_sequence_scenario: ${nb_pcap_file} file(s) to replay"


find "${test_case_path}" -name "*.pcap" | \
xargs -I {} bash -c "basename {}" | sort -t_ -k2 -n &> pcap_files.log

# TODO: try shuffle files: shuf --random-source=/dev/zero
# find ${test_case_path} -name "*.pcap" | sort -n | xargs -I {} bash -c "replay_capture_single_pcap {}"
# We sort by second field (-t_ -k2 -n) of result basename (and rebuild the full path in the function).
find "${test_case_path}" -name "*.pcap" | \
xargs -I {} bash -c "basename {}" | sort -t_ -k2 -n \
| parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "replay_capture_single_pcap {};" 

echo "replay_capture_test_sequence_scenario: test running finished"

# nb_tcpdump_processes=$(ps aux | grep tcpdump | grep "${sudo_tcpdump_pid_global}" | wc -l)
nb_tcpdump_processes=$(ps u --pid "${sudo_tcpdump_pid_global}" | tail +2 | wc -l)
echo "replay_capture_test_sequence_scenario: nb_tcpdump_processes: ${nb_tcpdump_processes}"

# Note: we deactivate failure on error because the process with pid=sudo_tcpdump_pid_global :
# * is often killed after the "pkill -15 -P [...]" command.
# * finishes often before the "wait [...]" command.
set +e

# Kill tcpdump (process with sudo's PID as parent)
sudo pkill -15 -P "${sudo_tcpdump_pid_global}"

# Kill sudo
# sudo kill -9 "${sudo_tcpdump_pid_global}"

echo "replay_capture_test_sequence_scenario: waiting ${sudo_tcpdump_pid_global}"
wait "${sudo_tcpdump_pid_global}"

set -e

echo "replay_capture_test_sequence_scenario: copying latest data"
cp "${pcap_all_path}" "${pcap_all_path_latest}"
rm -r "${output_pcap_directory_latest}"
cp -r "${output_pcap_directory_current}" "${output_pcap_directory_latest}"

echo "replay_capture_test_sequence_scenario: date: $(date)"

echo "replay_capture_test_sequence_scenario: end"











