


# TODO

* check how to reset easily


NB: on nicbox0



mkdir -p /home/jmazel/experimentations/nic_stack/chelsio_toe

cd /home/jmazel/experimentations/nic_stack/chelsio_toe


# 3.18.0.0

wget -c https://service.chelsio.com//store2/T5//Unified%20Wire%20\(NIC,TOE,vNIC,iWARP,iSCSI,FCoE,WD-UDP\)//Linux//ChelsioUwire-3.18.0.0/ChelsioUwire-3.18.0.0.tar.gz

tar xvf ChelsioUwire-3.18.0.0.tar.gz




# 3.19.0.3

wget -c https://service.chelsio.com/store2/T5/Unified%20Wire/Linux/ChelsioUwire-3.19.0.3/ChelsioUwire-3.19.0.3.tar.gz

tar xvf ChelsioUwire-3.19.0.3.tar.gz





# package installation

sudo apt install perl linux-headers-5.10.0-23-amd64 rdma-core libibverbs1 librdmacm1 librdmacm-dev libibverbs-dev ibverbs-utils rdmacm-utils libnl-3-dev libnl-route-3-dev libssl-dev bc automake autoconf flex bison libtool-bin cmake open-iscsi byacc targetcli-fb


sudo apt install perl linux-headers-amd64 rdma-core libibverbs1 librdmacm1 librdmacm-dev libibverbs-dev ibverbs-utils rdmacm-utils libnl-3-dev libnl-route-3-dev libssl-dev bc automake autoconf flex bison libtool-bin cmake open-iscsi byacc targetcli-fb



TODO: fix this
libibverbs devel packages are not installed on system.
Installing libibverbs & libibverbs-devel on System


sudo apt install perl linux-headers-6.1.0-33-amd64 dc rdma-core libibverbs1 librdmacm1 librdmacm-dev libibverbs-dev ibverbs-utils rdmacm-utils




# Installation - NIC/TOE

Chelsio-UnifiedWire-Linux-UserGuide.pdf



lspci | grep Ethernet


cd ~/experimentations/nic_stack/chelsio_toe/ChelsioUwire-3.19.0.3

sudo make toe_install

sudo reboot




# restart driver


sudo rmmod cxgb4

sudo modprobe cxgb4




sudo ip addr add 192.168.40.214/24 dev eth_chelsio_t520cr_0

sudo ip -6 addr add fd00:0:0:40::214/64 dev eth_chelsio_t520cr_0




sudo ip link set eth_chelsio_t520cr_0

sudo ip link set dev eth_chelsio_t520cr_0 up




sudo ip link set dev eth1 down

sudo ip link set dev eth1 up

sudo ip link set eth1 up






# firmware update

cd ~/experimentations/nic_stack/chelsio_toe/ChelsioUwire-3.19.0.3/Uboot/OptionROM/

sudo modprobe cxgb4

sudo cxgbtool eth1 loadboot clear

sudo cxgbtool eth1 loadboot cubt4.bin

sudo cxgbtool eth1 loadboot-cfg boot.cfg




# Setup module

sudo modprobe t4_tom unsupported_allow_unload=1

lsmod | grep cxgb4
lsmod | grep t4_tom

NB: no need to do "modprobe cxgb4" because "modprobe t4_tom" also loads cxgb4.


# Remove module

cat /sys/module/t4_tom/refcnt

## recnt = 0

sudo rmmod t4_tom
sudo rmmod cxgb4


## recnt > 0

policy_file_path=/home/jmazel/experimentations/nic_stack/chelsio_toe/policy_file

cop -o no-offload.cop "${policy_file_path}"

sudo cxgbtool eth1 policy no-offload.cop

sudo rmmod t4_tom
sudo rmmod toecore
sudo rmmod cxgb4






# DEBUG

0000:03:00.0 Ethernet controller: Chelsio Communications Inc T520-CR Unified Wire Ethernet Controller
0000:03:00.1 Ethernet controller: Chelsio Communications Inc T520-CR Unified Wire Ethernet Controller
0000:03:00.2 Ethernet controller: Chelsio Communications Inc T520-CR Unified Wire Ethernet Controller
0000:03:00.3 Ethernet controller: Chelsio Communications Inc T520-CR Unified Wire Ethernet Controller
0000:03:00.4 Ethernet controller: Chelsio Communications Inc T520-CR Unified Wire Ethernet Controller


https://unix.stackexchange.com/questions/658095/logical-device-name-assigned-to-physical-nic-mapping

ls -l /sys/class/net


sudo chdebug -b 0000:03:00.4



scp nicbox0:/home/jmazel/chelsio_debug_logs_with_cudbg_0.tar.bz2 .



cat debug_log/chelsio_debug_logs/chelsio_debug.log | grep -nr Airflow




## IP (if needed)


sudo ip addr add 192.168.40.214/24 dev eth_chelsio_t520cr_0

sudo ip -6 addr add fd00:0:0:40::214/64 dev eth_chelsio_t520cr_0


sudo ip link set eth_chelsio_t520cr_0

sudo ip link set dev eth_chelsio_t520cr_0 up
sudo ip link set dev eth1 up




# Check connection numbers


cat /sys/kernel/debug/cxgb4/<bus-id>/tids


sudo cat /sys/kernel/debug/cxgb4/0000:03:00.4/tids

check STID increase when echo server and nc used


cat /sys/module/t4_tom/refcnt




sudo ethtool -k eth_chelsio_t520cr_0 | grep ": on" | grep -v fixed





# echo

NB: sudo is needed because port 7 < 1024

sudo service xinetd stop

sudo ncat -e /bin/cat -k -l 7


sudo python3 ipv4_tcp_echo.py 0.0.0.0 7


# echo test


ping 192.168.40.214 


nc 192.168.40.214 7


nc fd00:0:0:40::214 7






nc 10.224.28.230 7





# IPv6


cat /sys/kernel/debug/cxgb4/<bus-id>/tids


sudo cat /sys/kernel/debug/cxgb4/0000:03:00.4/tids











