

# Network structure

All pair of base/target box/VM are located in vboxnet2 (to avoid being on the same default vboxnet0 network).
NB: if you use VirutalBox, you thus need to type at leats twice "VBoxManage hostonlyif create" to create vboxnet1 and vboxnet2.

All IP adresses are located in 192.168.56.0/24.
If the base machine has IP address 192.168.56.X, the target machine has an IP adress 192.168.56.X+1.
For example, for Debian 8, base uses 192.168.56.10 and target uses 192.168.56.11.

MAC adresses uses the same pattern.
For example, for Debian 8, base uses 000000000010 and target uses 000000000011.



# Configuration detail


## Debian 8

Base:   000000000010/192.168.56.10
Target: 000000000011/192.168.56.11


## FreeBSD 11.4

Base:   000000000012/192.168.56.12
Target: 000000000013/192.168.56.13


## OpenBSD 6.1

Base:   000000000014/192.168.56.14
Target: 000000000015/192.168.56.15


## Windows 10

Base:   000000000016/192.168.56.16
Target: 000000000017/192.168.56.17


## OpenBSD 5.7

Base:   000000000018/192.168.56.18
Target: 000000000019/192.168.56.19


## lwip 2.1.2

Situation is here a bit more complicated because the custom stack introduces two additional IP addresses (see the Vagrantfile).

Base:   000000000020/192.168.56.20
Target: 000000000021/192.168.56.21/fd00:0:0:56:0:0:0:21 (eth1)
        192.168.57.22/fd00:0:0:57:0:0:0:22 (tap0)
        192.168.57.23/fd00:0:0:57:0:0:0:23 (stack)


## µip 1.0

Situation is here (even) more complicated because 1) the custom stack introduces 
two additional IP addresses, and 2) uIP does not support i) dual stack and ii) two
instances on the same host (see the Vagrantfile).

Base:     000000000025/192.168.56.24
Targetv4: 000000000026/192.168.56.25 (eth1)
          192.168.57.26 (tap0)
          192.168.57.27 (stack)
Targetv6: 000000000026/192.168.56.28/fd00:0:0:56:0:0:0:28 (eth1)
          fd00:0:0:57:0:0:0:29 (tap0)
          fd00:0:0:57:0:0:0:30 (stack)

## Debian 12

Base:   000000000110/192.168.56.110
Target: 000000000111/192.168.56.111

## Debian 11

Base:   000000000032/192.168.56.32
Target: 000000000033/192.168.56.33


## Debian 10

Base:   000000000034/192.168.56.34
Target: 000000000035/192.168.56.35


## Debian 9

Base:   000000000036/192.168.56.36
Target: 000000000037/192.168.56.37


## OpenBSD 7.2

Base:   000000000038/192.168.56.38
Target: 000000000039/192.168.56.39


## Windows Server 2022

Base:   000000000040/192.168.56.40
Target: 000000000041/192.168.56.41


## Solaris 11.4

Base:   000000000042/192.168.56.42
Target: 000000000043/192.168.56.43


## MacOS 10.12

Base:   000000000044/192.168.56.44
Target: 000000000045/192.168.56.45


## Solaris 10

Base:   000000000046/192.168.56.46
Target: 000000000047/192.168.56.47


## FreeBSD 13

Base:   000000000048/192.168.56.48
Target: 000000000049/192.168.56.49


# Debian 11 router-firewall to 8

Base:   000000000040/192.168.56.50
Bridge: 000000000041/192.168.56.51 and 000000000041/192.168.56.52
Target: 000000000011/192.168.56.53



# Debian 8 router to 8

Base:   000000000040/192.168.56.50
Bridge: 000000000041/192.168.56.51 and 000000000041/192.168.56.52
Target: 000000000011/192.168.56.53


# Debian 9 router to 8

Base:   000000000040/192.168.56.54
Bridge: 000000000041/192.168.56.55 and 000000000041/192.168.56.56
Target: 000000000011/192.168.56.57


# Debian 10 router to 8

Base:   000000000040/192.168.56.60
Bridge: 000000000041/192.168.56.61 and 000000000041/192.168.56.62
Target: 000000000011/192.168.56.63


# Debian 11 router to 8

Base:   000000000040/192.168.56.60
Bridge: 000000000041/192.168.56.61 and 000000000041/192.168.56.62
Target: 000000000011/192.168.56.63





# Debian 8 bridge to 8

Base:   000000000040/192.168.56.80
Bridge: 000000000041/192.168.56.81 and 000000000041/192.168.56.82
Target: 000000000011/192.168.56.83


# Debian 9 bridge to 8

Base:   000000000040/192.168.56.84
Bridge: 000000000041/192.168.56.85 and 000000000041/192.168.56.86
Target: 000000000011/192.168.56.87


# Debian 10 bridge to 8

Base:   000000000040/192.168.56.90
Bridge: 000000000041/192.168.56.91 and 000000000041/192.168.56.92
Target: 000000000011/192.168.56.93


# Debian 11 bridge to 8

Base:   000000000040/192.168.56.90
Bridge: 000000000041/192.168.56.91 and 000000000041/192.168.56.92
Target: 000000000011/192.168.56.93




## picotcp 1.7.0

Situation is complicated because 1) the custom stack introduces 
two additional IP addresses, and 2) uIP does not support i) dual stack and ii) two
instances on the same host (see the Vagrantfile).

Base:     000000000100/192.168.56.100
Targetv4: 000000000101/192.168.56.101 (eth1)
          192.168.57.102 (tap0)
          192.168.57.103 (stack)
Targetv6: 000000000102/192.168.56.104/fd00:0:0:56:0:0:0:104 (eth1)
          fd00:0:0:57:0:0:0:105 (tap0)
          fd00:0:0:57:0:0:0:106 (stack)




# Command to extract relevant lines from Vagrantfiles

grep -nir "private_network" | grep "Vagrantfile:" | sort

