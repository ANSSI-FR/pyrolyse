#!/bin/bash

# set -xve

# PCAP_PATH=$1
NETWORK_INTERFACE=$1
OS_DIRECTORY=$2

# echo "test_icmp: PCAP_PATH: "$PCAP_PATH
echo "test_ip_fragmentation_icmp: NETWORK_INTERFACE: "$NETWORK_INTERFACE
echo "test_ip_fragmentation_icmp: OS_DIRECTORY: "$OS_DIRECTORY

printf -v date '%(%Y%m%d_%H%M%S)T' -1

output_pcap_directory_latest=$OS_DIRECTORY"/output_ip_fragmentation_icmp_pcap_latest"
export output_pcap_directory=$OS_DIRECTORY"/output_ip_fragmentation_icmp_pcap_"$date
mkdir -p $output_pcap_directory
mkdir -p $output_pcap_directory_latest

export NETWORK_INTERFACE

function test_ip_fragmentation_icmp_pcap() {
    echo ""
    echo ""
    echo ""
    echo "test_ip_fragmentation_icmp_pcap: start"
    pcap_path=$1
    pcap_path_wo_ext="${pcap_path%.*}"
    echo "test_ip_fragmentation_icmp_pcap: pcap_path: "$pcap_path
    echo "test_ip_fragmentation_icmp_pcap: pcap_path_wo_ext: "$pcap_path_wo_ext
    
    test_index=`basename $pcap_path_wo_ext | cut -d'_' -f2`
    echo "test_ip_fragmentation_icmp_pcap: test_index: "$test_index
    pcap_test_path=$output_pcap_directory"/vm0_ip_fragmentation_icmp_"$test_index".pcap"
    echo "test_ip_fragmentation_icmp_pcap: pcap_test_path: "$pcap_test_path
    
    # We filter TTL exceeded message because they sometimes appear in traces.
    # sudo tcpdump -i $NETWORK_INTERFACE 'icmp' -w $pcap_test_path &
    # sleep 10
    sudo tcpdump -i $NETWORK_INTERFACE 'icmp and icmp[icmptype] != icmp-timxceed' -w $pcap_test_path &
    sudo_tcpdump_pid=$!
    ps aux | grep tcpdump
    echo "test_ip_fragmentation_icmp_pcap: sudo_tcpdump_pid: "$sudo_tcpdump_pid
    
    sleep 2
    echo "test_ip_fragmentation_icmp_pcap: launching tcpreplay"
    sudo tcpreplay -i $NETWORK_INTERFACE -t $pcap_path

    sleep 2
    echo "test_ip_fragmentation_icmp_pcap: before kill -15 on tcpdump"
    ps aux | grep tcpdump
    # sudo pkill tcpdump
    # Kill tcpdump (process with sudo's PID as parent)
    sudo pkill -15 -P $sudo_tcpdump_pid
    echo "test_ip_fragmentation_icmp_pcap: after kill -15 on tcpdump"

    # Kill sudo
    echo "test_ip_fragmentation_icmp_pcap: before kill -9 on tcpdump"
    sudo kill -9 $sudo_tcpdump_pid
    echo "test_ip_fragmentation_icmp_pcap: after kill -9 on tcpdump"
    ps aux | grep tcpdump
    # sudo kill -2 $tcpdump_pid
    # echo "analyze_pbes_result: after kill -2"
    # ps aux | grep tcpdump
    
    echo "test_ip_fragmentation_icmp_pcap: end"
}
export -f test_ip_fragmentation_icmp_pcap

sudo pkill tcpdump
sudo pkill tcpreplay
    
pcap_all_path_latest=$OS_DIRECTORY"/ip_fragmentation_icmp_all_latest.pcap"
pcap_all_path=$OS_DIRECTORY"/ip_fragmentation_icmp_all_"$date".pcap"
sudo tcpdump -i $NETWORK_INTERFACE 'icmp' -w $pcap_all_path &
sudo_tcpdump_pid=$!
    
# find $PCAP_PATH -name "*.pcap"
find $OS_DIRECTORY/ip_fragmentation_icmp_pcap -name "*.pcap" | sort | xargs -I {} bash -c "test_ip_fragmentation_icmp_pcap {}"

# Kill tcpdump (process with sudo's PID as prent)
sudo pkill -15 -P $sudo_tcpdump_pid
# Kill sudo
sudo kill -9 $sudo_tcpdump_pid

wait $sudo_tcpdump_pid

# Copying as the latest
cp $pcap_all_path $pcap_all_path_latest
rm -r $output_pcap_directory_latest
cp -r $output_pcap_directory $output_pcap_directory_latest













