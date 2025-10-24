#!/bin/bash

set -x
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

suricata_log_tmp_directory_path=$OS_DIRECTORY/suricata_tmp_$date
mkdir -p $suricata_log_tmp_directory_path
conf_dir=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/conf

protocol=`echo $INPUT_DIRECTORY | cut -d '_' -f1`
scenario=`echo $INPUT_DIRECTORY | cut -d '_' -f2`

export OS_DIRECTORY
export INPUT_DIRECTORY
export suricata_log_tmp_directory_path
export MIDPOINT_PATH
export conf_dir
export protocol
export scenario

echo "replay_capture_test_sequence: suricata.yaml modification"
sed -i -e "s~^classification-file:.*~classification-file: $MIDPOINT_PATH/etc/suricata/classification.config~g" $conf_dir/suricata.yaml
sed -i -e "s~^reference-config-file:.*~reference-config-file: $MIDPOINT_PATH/etc/suricata/reference.config~g" $conf_dir/suricata.yaml


function replay_single_pcap_copy_log() {
    echo ""
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


    if [[ "$protocol" == "ip" ]]
    then  
        rules_file=$conf_dir/suricata.rules
    elif [[ "$scenario" == "peos" ]]
    then
        #rules_file=$conf_dir/suricata_tcp_peos.rules
        rules_file=$conf_dir/suricata.rules
    else
        #rules_file=$conf_dir/suricata_tcp_pep.rules
        rules_file=$conf_dir/suricata.rules
    fi
    
    $MIDPOINT_PATH/bin/suricata \
    -r $pcap_path \
    -c $conf_dir/suricata.yaml \
    -S $rules_file \
    -l $suricata_log_tmp_directory_path


    echo "replay_single_pcap_copy_log: start"
    #if [[ "$protocol" == "ip" ]]
    #then  
    #    mv $suricata_log_tmp_directory_path/eve.json $log_path
    #else 
    #    mv $suricata_log_tmp_directory_path/tcp-data.log $log_path
    #fi

    mv $suricata_log_tmp_directory_path/eve.json $log_path
    rm $suricata_log_tmp_directory_path/*
    
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

echo "Finished!!!!"
date











