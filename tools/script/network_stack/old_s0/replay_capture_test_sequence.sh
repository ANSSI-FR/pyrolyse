#!/bin/bash

# set -xve

# PCAP_PATH=$1
NETWORK_INTERFACE=$1
TCMDUMP_FILTER=$2
OS_DIRECTORY=$3
INPUT_DIRECTORY=$4

OS_DIRECTORY_PATH=target/os/$OS_DIRECTORY

echo "replay_capture_test_sequence: NETWORK_INTERFACE: "$NETWORK_INTERFACE
echo "replay_capture_test_sequence: TCMDUMP_FILTER: "$TCMDUMP_FILTER
echo "replay_capture_test_sequence: OS_DIRECTORY_PATH: "$OS_DIRECTORY_PATH
echo "replay_capture_test_sequence: INPUT_DIRECTORY: "$INPUT_DIRECTORY

printf -v date '%(%Y%m%d_%H%M%S)T' -1

output_pcap_directory_latest=$OS_DIRECTORY_PATH"/output_"$INPUT_DIRECTORY"_latest"
output_pcap_directory=$OS_DIRECTORY_PATH"/output_"$INPUT_DIRECTORY"_"$date
mkdir -p $output_pcap_directory
mkdir -p $output_pcap_directory_latest

export OS_DIRECTORY_PATH
export OS_DIRECTORY
export INPUT_DIRECTORY
export NETWORK_INTERFACE
export TCMDUMP_FILTER
export output_pcap_directory

function replay_capture_single_pcap() {
    echo ""
    echo ""
    echo ""
    echo "replay_capture_single_pcap: start"
    pcap_name=$1
    pcap_path=$OS_DIRECTORY_PATH/$INPUT_DIRECTORY/$pcap_name
    pcap_path_wo_ext="${pcap_path%.*}"
    echo "replay_capture_single_pcap: pcap_path: "$pcap_path
    echo "replay_capture_single_pcap: pcap_path_wo_ext: "$pcap_path_wo_ext
    
    test_index=`basename $pcap_path_wo_ext | cut -d'_' -f2`
    echo "replay_capture_single_pcap: test_index: "$test_index
    pcap_test_path=$output_pcap_directory"/base_sequence_"$test_index".pcap"
    echo "replay_capture_single_pcap: pcap_test_path: "$pcap_test_path
    
    # We filter TTL exceeded message because they sometimes appear in traces.
    # sudo tcpdump -i $NETWORK_INTERFACE 'icmp' -w $pcap_test_path &
    # sleep 10
    echo "replay_capture_single_pcap: launching answering traffic capture using tcpdump"
    sudo tcpdump -i $NETWORK_INTERFACE "$TCMDUMP_FILTER" -w $pcap_test_path &
    sudo_tcpdump_pid=$!
    ps aux | grep tcpdump
    echo "replay_capture_single_pcap: sudo_tcpdump_pid: "$sudo_tcpdump_pid
    
    sleep 2
    echo "replay_capture_single_pcap: replaying trace with tcpreplay"
    sudo tcpreplay -i $NETWORK_INTERFACE -t $pcap_path

    sleep 2
    echo "replay_capture_single_pcap: before kill -15 on tcpdump"
    ps aux | grep tcpdump
    # sudo pkill tcpdump
    # Kill tcpdump (process with sudo's PID as parent)
    sudo pkill -15 -P $sudo_tcpdump_pid
    echo "replay_capture_single_pcap: after kill -15 on tcpdump"

    # Kill sudo
    echo "replay_capture_single_pcap: before kill -9 on tcpdump"
    sudo kill -9 $sudo_tcpdump_pid
    echo "replay_capture_single_pcap: after kill -9 on tcpdump"
    ps aux | grep tcpdump
    # sudo kill -2 $tcpdump_pid
    # echo "analyze_pbes_result: after kill -2"
    # ps aux | grep tcpdump
    
    echo "replay_capture_single_pcap: end"
}
export -f replay_capture_single_pcap

sudo pkill tcpdump
sudo pkill tcpreplay
    
pcap_all_path_latest=$OS_DIRECTORY_PATH"/"$INPUT_DIRECTORY"_all_latest.pcap"
pcap_all_path=$OS_DIRECTORY_PATH"/"$INPUT_DIRECTORY"_all_"$date".pcap"
sudo tcpdump -i $NETWORK_INTERFACE "$TCMDUMP_FILTER" -w $pcap_all_path &
sudo_tcpdump_pid=$!
    
# find $PCAP_PATH -name "*.pcap"
find $OS_DIRECTORY_PATH/$INPUT_DIRECTORY -name "*.pcap" > pcap_files_log
# find $OS_DIRECTORY_PATH/$INPUT_DIRECTORY -name "*.pcap" | sort -n | xargs -I {} bash -c "replay_capture_single_pcap {}"
# We sort by second field (-t_ -k2 -n) of result basename (and rebuild the full path in the function).
find $OS_DIRECTORY_PATH/$INPUT_DIRECTORY -name "*.pcap" | xargs -I {} bash -c "basename {}" | sort -t_ -k2 -n | xargs -I {} bash -c "replay_capture_single_pcap {}"

# Kill tcpdump (process with sudo's PID as prent)
sudo pkill -15 -P $sudo_tcpdump_pid
# Kill sudo
sudo kill -9 $sudo_tcpdump_pid

wait $sudo_tcpdump_pid

# Copying as the latest
cp $pcap_all_path $pcap_all_path_latest
rm -r $output_pcap_directory_latest
cp -r $output_pcap_directory $output_pcap_directory_latest

echo "Finished!!!!"
date











