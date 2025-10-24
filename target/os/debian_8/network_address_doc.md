

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

Base:   000000000014/192.168.56.18
Target: 000000000015/192.168.56.19


## lwip 2.1.2

Situation is here a bit more complicated because the custom stack has an additional IP address (see the Vagrantfile).

Base:   000000000020/192.168.56.20
Target: 000000000021/192.168.56.21 and 192.168.56.23


## µip 1.0

Situation is here a bit more complicated because the custom stack has an additional IP address (see the Vagrantfile).

Base:   000000000025/192.168.56.25
Target: 000000000026/192.168.56.26 and 192.168.56.27



## Debian 11

Base:   000000000010/192.168.56.32
Target: 000000000011/192.168.56.33


## Debian 10

Base:   000000000010/192.168.56.34
Target: 000000000011/192.168.56.35


## Debian 9

Base:   000000000010/192.168.56.36
Target: 000000000011/192.168.56.37

## OpenBSD 7.2

Base:   000000000014/192.168.56.38
Target: 000000000015/192.168.56.39

## Windows Server 2022

Base:   000000000014/192.168.56.40
Target: 000000000015/192.168.56.41

## Solaris 7.4

Base:   000000000014/192.168.56.42
Target: 000000000015/192.168.56.43

## MacOS 10.12

Base:   000000000014/192.168.56.44
Target: 000000000015/192.168.56.45


## Solaris 10

Base:   000000000010/192.168.56.46
Target: 000000000011/192.168.56.47



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






# Command to extract relevant lines from Vagrantfiles

grep -nir "private_network" | grep "Vagrantfile:" | sort

