#!/bin/bash

set -eu
set -o pipefail

echo "replay_capture_test_sequence_scenario: start"

network_interface=$1
tcpdump_filter_global=$2
tcpdump_filter_single=$3
target_directory_path=$4
input_directory=$5
nb_processes=$6

echo "replay_capture_test_sequence_scenario: network_interface: ${network_interface}"
echo "replay_capture_test_sequence_scenario: tcpdump_filter_global: ${tcpdump_filter_global}"
echo "replay_capture_test_sequence_scenario: tcpdump_filter_single: ${tcpdump_filter_single}"
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
export target_directory_path
export output_pcap_directory_current

export test_case_path

function replay_capture_single_pcap() {
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

    # We filter TTL exceeded message because they sometimes appear in traces.
    # sudo tcpdump -i "${network_interface_name}" 'icmp' -w "${pcap_test_path}" &
    # sleep 10
    echo "replay_capture_single_pcap: launching answering traffic capture using tcpdump"
    set -x
    sudo tcpdump -i "${network_interface_name}" "${tcpdump_filter_single_expanded}" -w "${pcap_test_path}" &
    sudo_tcpdump_pid_single=$!
    set +x
    # ps aux | grep tcpdump
    echo "replay_capture_single_pcap: sudo_tcpdump_pid_single: ${sudo_tcpdump_pid_single}"
    
    sleep 2
    echo "replay_capture_single_pcap: replaying trace with tcpreplay"
    sudo tcpreplay -i "${network_interface_name}" -t "${pcap_path}"

    sleep 2
    # ps aux | grep tcpdump
    # TODO: test 'ps u -C tcpdump'
    # TODO: test 'ps u --pid "${sudo_tcpdump_pid_single}" | tail +2'
    # nb_tcpdump_processes=$(ps aux | grep tcpdump | grep "${sudo_tcpdump_pid_single}" | wc -l)
    nb_tcpdump_processes=$(ps u --pid "${sudo_tcpdump_pid_single}" | tail +2 | wc -l)
    echo "replay_capture_single_pcap: nb_tcpdump_processes before kill -15 -P on tcpdump: ${nb_tcpdump_processes}"
    # sudo pkill tcpdump
    # Kill tcpdump (process with sudo's PID as parent)
    sudo pkill -15 -P "${sudo_tcpdump_pid_single}"
    nb_processes=$(ps aux | grep tcpdump | grep "${sudo_tcpdump_pid_single}" | wc -l)
    echo "replay_capture_single_pcap: nb_processes after kill -15 -P on tcpdump: ${nb_processes}"

    # Kill sudo
    # nb_tcpdump_processes=$(ps aux | grep tcpdump | grep "${sudo_tcpdump_pid_single}" | wc -l)
    nb_tcpdump_processes=$(ps u --pid "${sudo_tcpdump_pid_single}" | tail +2 | wc -l)
    echo "replay_capture_single_pcap: nb_tcpdump_processes before kill -9 on tcpdump: ${nb_tcpdump_processes}"
    sudo kill -9 "${sudo_tcpdump_pid_single}"
    # nb_tcpdump_processes=$(ps aux | grep tcpdump | grep "${sudo_tcpdump_pid_single}" | wc -l)
    nb_tcpdump_processes=$(ps u --pid "${sudo_tcpdump_pid_single}" | tail +2 | wc -l)
    echo "replay_capture_single_pcap: nb_tcpdump_processes after kill -9 on tcpdump: ${nb_tcpdump_processes}"
    # ps aux | grep tcpdump
    # sudo kill -2 $tcpdump_pid
    # echo "analyze_pbes_result: after kill -2"
    # ps aux | grep tcpdump
    
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

sudo tcpdump -i "${network_interface_name}" "${tcpdump_filter_global}" -w "${pcap_all_path}" &
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
sudo kill -9 "${sudo_tcpdump_pid_global}"

echo "replay_capture_test_sequence_scenario: waiting ${sudo_tcpdump_pid_global}"
wait "${sudo_tcpdump_pid_global}"

set -e

echo "replay_capture_test_sequence_scenario: copying latest data"
cp "${pcap_all_path}" "${pcap_all_path_latest}"
rm -r "${output_pcap_directory_latest}"
cp -r "${output_pcap_directory_current}" "${output_pcap_directory_latest}"

echo "replay_capture_test_sequence_scenario: date: $(date)"

echo "replay_capture_test_sequence_scenario: end"











