

NB: on nicbox0



# Settingup directory


rm -r /home/jmazel/experimentations/nic_stack/chelsio_toe

mkdir -p /home/jmazel/experimentations/nic_stack/chelsio_toe

cd /home/jmazel/experimentations/nic_stack/chelsio_toe

wget -c https://service.chelsio.com//store2/T5//Unified%20Wire%20\(NIC,TOE,vNIC,iWARP,iSCSI,FCoE,WD-UDP\)//Linux//ChelsioUwire-3.18.0.0/ChelsioUwire-3.18.0.0.tar.gz

tar xvf ChelsioUwire-3.18.0.0.tar.gz



# Installing dependencies


sudo apt install perl linux-headers-amd64 rdma-core libibverbs1 librdmacm1 librdmacm-dev libibverbs-dev ibverbs-utils rdmacm-utils libnl-3-dev libnl-route-3-dev libssl-dev bc automake autoconf flex bison libtool-bin cmake open-iscsi byacc targetcli-fb



# Test


sudo tcpdump -i eth1





# IPv6


P271


## Compilation and installation

cd /home/jmazel/experimentations/nic_stack/chelsio_toe/ChelsioUwire-3.18.0.0

sudo make install

sudo reboot




## Setup


sudo rmmod csiostor cxgb4i cxgbit iw_cxgb4 chcr cxgb4vf cxgb4 libcxgbi libcxgb

sudo modprobe cxgb4

sudo modprobe t4_tom

sudo ifconfig eth_chelsio_t520cr_0 up



sudo cat /sys/kernel/debug/cxgb4/0000:03:00.4/tids






# TCP


P46


## Compilation and installation

cd /home/jmazel/experimentations/nic_stack/chelsio_toe/ChelsioUwire-3.18.0.0

sudo make toe_install

sudo reboot





## Setup


sudo modprobe t4_tom

sudo ifconfig eth_chelsio_t520cr_0 up

sudo cat /sys/kernel/debug/cxgb4/0000:03:00.4/tids




sudo ethtool -k eth_chelsio_t520cr_0 | grep ": on" | grep -v fixed



sudo ifconfig eth_chelsio_t520cr_0 192.168.40.214 up




## echo

NB: xinetd does not use offloading (TCP packets are visible in tcpdump)

sudo service xinetd stop

sudo ncat -e /bin/cat -k -l 7



nc 192.168.40.214 7


nc fd00:0:0:40::214 7





ping fd00:0:0:40::214






# UDP -- TODO: check


P261


## Compilation and installation

cd /home/jmazel/experimentations/nic_stack/chelsio_toe/ChelsioUwire-3.18.0.0

sudo make udp_offload_install

sudo reboot





## Setup

sudo rmmod csiostor cxgb4i cxgbit iw_cxgb4 chcr cxgb4vf cxgb4 libcxgbi libcxgb

sudo modprobe t4_tom

sudo ifconfig eth_chelsio_t520cr_0 up

sudo cat /sys/kernel/debug/cxgb4/0000:03:00.4/tids




sudo ethtool -k eth_chelsio_t520cr_0 | grep ": on" | grep -v fixed



sudo ifconfig eth_chelsio_t520cr_0 192.168.40.214 up




## echo

sudo service xinetd stop

sudo ncat -e /bin/cat -k -l 7



nc 192.168.40.214 7


nc fd00:0:0:40::214 7





ping fd00:0:0:40::214





