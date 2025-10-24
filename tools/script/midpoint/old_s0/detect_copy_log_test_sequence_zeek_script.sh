#!/bin/bash

set -x
# set -v
set -e

OS_DIRECTORY=$1
INPUT_DIRECTORY=$2
MIDPOINT_PATH=$3

echo "replay_capture_test_sequence: OS_DIRECTORY: "$OS_DIRECTORY
echo "replay_capture_test_sequence: INPUT_DIRECTORY: "$INPUT_DIRECTORY
echo "replay_capture_test_sequence: MIDPOINT_PATH: "$MIDPOINT_PATH

printf -v date '%(%Y%m%d_%H%M%S)T' -1

output_log_directory_latest=$OS_DIRECTORY"/output_"$INPUT_DIRECTORY"_latest"
export output_log_directory=$OS_DIRECTORY"/output_"$INPUT_DIRECTORY"_"$date
mkdir -p $output_log_directory_latest
mkdir -p $output_log_directory

zeek_log_tmp_directory_path=$OS_DIRECTORY/zeek_tmp_$date
mkdir -p $zeek_log_tmp_directory_path

conf_dir=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/conf

protocol=`echo $INPUT_DIRECTORY | cut -d '_' -f1`

export OS_DIRECTORY
export INPUT_DIRECTORY
export MIDPOINT_PATH
export conf_dir
export zeek_log_tmp_directory_path
export protocol

function replay_single_pcap_copy_log() {
    echo ""-s
    echo ""
    echo ""
    echo "replay_single_pcap_copy_log: start"
    pcap_name=$1
    pcap_path=$OS_DIRECTORY/$INPUT_DIRECTORY/$pcap_name
    pcap_path_wo_ext="${pcap_path%.*}"
    echo "replay_single_pcap_copy_log: pcap_path: "$pcap_path
    echo "replay_single_pcap_copy_log: pcap_path_wo_ext: "$pcap_path_wo_ext
    
    test_index=`basename $pcap_path_wo_ext | cut -d'_' -f2`
    echo "replay_single_pcap_copy_log: test_index: "$test_index
    log_path=$output_log_directory"/base_sequence_"$test_index".log"
    echo "replay_single_pcap_copy_log: log_path: "$log_path

	if [ "$protocol" = "ipv4" ] || [ "$protocol" = "ipv6" ]; then
        script_file=$conf_dir/icmp_echo_request_payload.zeek
    elif [ "$protocol" = "tcp" ]; then
        script_file=$conf_dir/tcp.zeek
    fi
    
    echo "replay_single_pcap_copy_log: script_file: "$script_file

    $MIDPOINT_PATH/bin/zeek \
    -r $pcap_path \
    $script_file \
    Log::default_logdir=$zeek_log_tmp_directory_path

    ls $zeek_log_tmp_directory_path

    echo "replay_single_pcap_copy_log: handling notice.log"
    if [ -f $zeek_log_tmp_directory_path/notice.log ]; then
        mv $zeek_log_tmp_directory_path/notice.log $log_path
    else
        # If notice.log does not exist, we create an empty file.
        touch $log_path
    fi

    rm $zeek_log_tmp_directory_path/*
    
    echo "replay_single_pcap_copy_log: end"
}
export -f replay_single_pcap_copy_log

    
# find $PCAP_PATH -name "*.pcap"
find $OS_DIRECTORY/$INPUT_DIRECTORY -name "*.pcap" > pcap_files_log
# find $OS_DIRECTORY/$INPUT_DIRECTORY -name "*.pcap" | sort -n | xargs -I {} bash -c "replay_capture_single_pcap {}"
# We sort by second field (-t_ -k2 -n) of result basename (and rebuild the full path in the function).
find $OS_DIRECTORY/$INPUT_DIRECTORY -name "*.pcap" | xargs -I {} bash -c "basename {}" | sort -t_ -k2 -n | xargs -I {} bash -c "replay_single_pcap_copy_log {}"

# Copying as the latest
rm -r $output_log_directory_latest
cp -r $output_log_directory $output_log_directory_latest
rm -r $zeek_log_tmp_directory_path

echo "Finished!!!!"
date











