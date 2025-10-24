These environment variables must be initialized in the terminal instance that will be used to perform the IDS reassembly policy testing.

# Zeek

## IPv4 and IPv6 

### signature

export target_ipv4_addr="192.168.10.50"
export target_ipv6_addr="fe80::200:ff:fe02:50"
export target_reassembly_policy="zeek_signature"
export export reassembly_options="default"

### script

export target_ipv4_addr="192.168.10.50"
export target_ipv6_addr="fe80::200:ff:fe02:50"
export target_reassembly_policy="zeek_script"
export reassembly_options="default"

## TCP 

### signature

export target_ipv4_addr="192.168.10.50"
export target_reassembly_policy="zeek_signature"
export reassembly_options="default"

### script

export target_ipv4_addr="192.168.10.50"
export target_reassembly_policy="zeek_script"
export reassembly_options="default"

# Suricata

## IPv4 and IPv6 

### icmp-default rule file

#### suricata_windows

export target_ipv4_addr="192.168.10.50"
export target_ipv6_addr="fe80::200:ff:fe02:50"
export target_reassembly_policy="suricata_windows"
export reassembly_options="icmp-default"

#### suricata_bsd

export target_ipv4_addr="192.168.10.51"
export target_ipv6_addr="fe80::200:ff:fe02:51"
export target_reassembly_policy="suricata_bsd"
export reassembly_options="icmp-default"

#### suricata_bsd_right

export target_ipv4_addr="192.168.10.52"
export target_ipv6_addr="fe80::200:ff:fe02:52"
export target_reassembly_policy="suricata_bsd_right"
export reassembly_options="icmp-default"

#### suricata_old_linux

export target_ipv4_addr="192.168.10.53"
export target_ipv6_addr="fe80::200:ff:fe02:53"
export target_reassembly_policy="suricata_old_linux"
export reassembly_options="icmp-default"

#### suricata_linux

export target_ipv4_addr="192.168.10.54"
export target_ipv6_addr="fe80::200:ff:fe02:54"
export target_reassembly_policy="suricata_linux"
export reassembly_options="icmp-default"

#### suricata_old_solaris

export target_ipv4_addr="192.168.10.55"
export target_ipv6_addr="fe80::200:ff:fe02:55"
export target_reassembly_policy="suricata_old_solaris"
export reassembly_options="icmp-default"

#### suricata_solaris

export target_ipv4_addr="192.168.10.56"
export target_ipv6_addr="fe80::200:ff:fe02:56"
export target_reassembly_policy="suricata_solaris"
export reassembly_options="icmp-default"

#### suricata_hpux10

export target_ipv4_addr="192.168.10.57"
export target_ipv6_addr="fe80::200:ff:fe02:57"
export target_reassembly_policy="suricata_hpux10"
export reassembly_options="icmp-default"

#### suricata_hpux11

export target_ipv4_addr="192.168.10.58"
export target_ipv6_addr="fe80::200:ff:fe02:58"
export target_reassembly_policy="suricata_hpux11"
export reassembly_options="icmp-default"

#### suricata_irix

export target_ipv4_addr="192.168.10.59"
export target_ipv6_addr="fe80::200:ff:fe02:59"
export target_reassembly_policy="suricata_irix"
export reassembly_options="icmp-default"

#### suricata_macos

export target_ipv4_addr="192.168.10.60"
export target_ipv6_addr="fe80::200:ff:fe02:60"
export target_reassembly_policy="suricata_macos"
export reassembly_options="icmp-default"

#### suricata_vista

export target_ipv4_addr="192.168.10.61"
export target_ipv6_addr="fe80::200:ff:fe02:61"
export target_reassembly_policy="suricata_vista"
export reassembly_options="icmp-default"

#### suricata_windows2k3

export target_ipv4_addr="192.168.10.62"
export target_ipv6_addr="fe80::200:ff:fe02:62"
export target_reassembly_policy="suricata_windows2k3"
export reassembly_options="icmp-default"

#### suricata_default

export target_ipv4_addr="192.168.10.63"
export target_ipv6_addr="fe80::200:ff:fe02:63"
export target_reassembly_policy="suricata_default"
export reassembly_options="icmp-default"


### flow-only-frag rule file

#### suricata_windows

export target_ipv4_addr="192.168.10.50"
export target_ipv6_addr="fe80::200:ff:fe02:50"
export target_reassembly_policy="suricata_windows"
export reassembly_options="flow-only-frag"

#### suricata_bsd

export target_ipv4_addr="192.168.10.51"
export target_ipv6_addr="fe80::200:ff:fe02:51"
export target_reassembly_policy="suricata_bsd"
export reassembly_options="flow-only-frag"

#### suricata_bsd_right

export target_ipv4_addr="192.168.10.52"
export target_ipv6_addr="fe80::200:ff:fe02:52"
export target_reassembly_policy="suricata_bsd_right"
export reassembly_options="flow-only-frag"

#### suricata_old_linux

export target_ipv4_addr="192.168.10.53"
export target_ipv6_addr="fe80::200:ff:fe02:53"
export target_reassembly_policy="suricata_old_linux"
export reassembly_options="flow-only-frag"

#### suricata_linux

export target_ipv4_addr="192.168.10.54"
export target_ipv6_addr="fe80::200:ff:fe02:54"
export target_reassembly_policy="suricata_linux"
export reassembly_options="flow-only-frag"

#### suricata_old_solaris

export target_ipv4_addr="192.168.10.55"
export target_ipv6_addr="fe80::200:ff:fe02:55"
export target_reassembly_policy="suricata_old_solaris"
export reassembly_options="flow-only-frag"

#### suricata_solaris

export target_ipv4_addr="192.168.10.56"
export target_ipv6_addr="fe80::200:ff:fe02:56"
export target_reassembly_policy="suricata_solaris"
export reassembly_options="flow-only-frag"

#### suricata_hpux10

export target_ipv4_addr="192.168.10.57"
export target_ipv6_addr="fe80::200:ff:fe02:57"
export target_reassembly_policy="suricata_hpux10"
export reassembly_options="flow-only-frag"

#### suricata_hpux11

export target_ipv4_addr="192.168.10.58"
export target_ipv6_addr="fe80::200:ff:fe02:58"
export target_reassembly_policy="suricata_hpux11"
export reassembly_options="flow-only-frag"

#### suricata_irix

export target_ipv4_addr="192.168.10.59"
export target_ipv6_addr="fe80::200:ff:fe02:59"
export target_reassembly_policy="suricata_irix"
export reassembly_options="flow-only-frag"

#### suricata_macos

export target_ipv4_addr="192.168.10.60"
export target_ipv6_addr="fe80::200:ff:fe02:60"
export target_reassembly_policy="suricata_macos"
export reassembly_options="flow-only-frag"

#### suricata_vista

export target_ipv4_addr="192.168.10.61"
export target_ipv6_addr="fe80::200:ff:fe02:61"
export target_reassembly_policy="suricata_vista"
export reassembly_options="flow-only-frag"

#### suricata_windows2k3

export target_ipv4_addr="192.168.10.62"
export target_ipv6_addr="fe80::200:ff:fe02:62"
export target_reassembly_policy="suricata_windows2k3"
export reassembly_options="flow-only-frag"

#### suricata_default

export target_ipv4_addr="192.168.10.63"
export target_ipv6_addr="fe80::200:ff:fe02:63"
export target_reassembly_policy="suricata_default"
export reassembly_options="flow-only-frag"


## TCP 

### flow-only-stream rule file

#### suricata_windows

export target_ipv4_addr="192.168.10.50"
export target_ipv6_addr="fe80::200:ff:fe02:50"
export target_reassembly_policy="suricata_windows"
export reassembly_options="flow-only-stream"

#### suricata_bsd

export target_ipv4_addr="192.168.10.51"
export target_reassembly_policy="suricata_bsd"
export reassembly_options="flow-only-stream"
export target_os="openbsd_7.4"

#### suricata_bsd_right

export target_ipv4_addr="192.168.10.52"
export target_reassembly_policy="suricata_bsd_right"
export reassembly_options="flow-only-stream"

#### suricata_old_linux

export target_ipv4_addr="192.168.10.53"
export target_reassembly_policy="suricata_old_linux"
export reassembly_options="flow-only-stream"

#### suricata_linux

export target_ipv4_addr="192.168.10.54"
export target_reassembly_policy="suricata_linux"
export reassembly_options="flow-only-stream"
export target_os="debian_12"

#### suricata_old_solaris

export target_ipv4_addr="192.168.10.55"
export target_reassembly_policy="suricata_old_solaris"
export reassembly_options="flow-only-stream"

#### suricata_solaris

export target_ipv4_addr="192.168.10.56"
export target_reassembly_policy="suricata_solaris"
export reassembly_options="flow-only-stream"
export target_os="solaris_11.2"

#### suricata_hpux10

export target_ipv4_addr="192.168.10.57"
export target_reassembly_policy="suricata_hpux10"
export reassembly_options="flow-only-stream"

#### suricata_hpux11

export target_ipv4_addr="192.168.10.58"
export target_reassembly_policy="suricata_hpux11"
export reassembly_options="flow-only-stream"

#### suricata_irix

export target_ipv4_addr="192.168.10.59"
export target_reassembly_policy="suricata_irix"
export reassembly_options="flow-only-stream"

#### suricata_macos

export target_ipv4_addr="192.168.10.60"
export target_reassembly_policy="suricata_macos"
export reassembly_options="flow-only-stream"

#### suricata_vista

export target_ipv4_addr="192.168.10.61"
export target_reassembly_policy="suricata_vista"
export reassembly_options="flow-only-stream"
export target_os="windows_10"

#### suricata_windows2k3

export target_ipv4_addr="192.168.10.62"
export target_reassembly_policy="suricata_windows2k3"
export reassembly_options="flow-only-stream"

#### suricata_default

export target_ipv4_addr="192.168.10.63"
export target_reassembly_policy="suricata_default"
export reassembly_options="flow-only-stream"

# Snort

## IPv4 and IPv6 

### icmp-default rule file


#### snort_linux

export target_ipv4_addr="192.168.10.64"
export target_ipv6_addr="fe80::200:ff:fe02:64"
export target_reassembly_policy="snort_linux"
export reassembly_options="icmp-default"
export target_os="debian_12"

#### snort_windows

export target_ipv4_addr="192.168.10.65"
export target_ipv6_addr="fe80::200:ff:fe02:65"
export target_reassembly_policy="snort_windows"
export reassembly_options="icmp-default"
export target_os="windows_10"

#### snort_bsd

export target_ipv4_addr="192.168.10.66"
export target_ipv6_addr="fe80::200:ff:fe02:66"
export target_reassembly_policy="snort_bsd"
export reassembly_options="icmp-default"
export target_os="freebsd_13"

#### snort_bsd_right

export target_ipv4_addr="192.168.10.67"
export target_ipv6_addr="fe80::200:ff:fe02:67"
export target_reassembly_policy="snort_bsd_right"
export reassembly_options="icmp-default"

#### snort_solaris

export target_ipv4_addr="192.168.10.68"
export target_ipv6_addr="fe80::200:ff:fe02:68"
export target_reassembly_policy="snort_solaris"
export reassembly_options="icmp-default"
export target_os="solaris_11.2"

#### snort_last

export target_ipv4_addr="192.168.10.69"
export target_ipv6_addr="fe80::200:ff:fe02:69"
export target_reassembly_policy="snort_last"
export reassembly_options="icmp-default"

#### snort_first

export target_ipv4_addr="192.168.10.70"
export target_ipv6_addr="fe80::200:ff:fe02:70"
export target_reassembly_policy="snort_first"
export reassembly_options="icmp-default"

#### snort_default

export target_ipv4_addr="192.168.10.71"
export target_ipv6_addr="fe80::200:ff:fe02:71"
export target_reassembly_policy="snort_default"
export reassembly_options="icmp-default"

### flow-only-frag rule file

#### snort_linux

export target_ipv4_addr="192.168.10.64"
export target_ipv6_addr="fe80::200:ff:fe02:64"
export target_reassembly_policy="snort_linux"
export reassembly_options="flow-only-frag"
export target_os="debian_12"

#### snort_windows

export target_ipv4_addr="192.168.10.65"
export target_ipv6_addr="fe80::200:ff:fe02:65"
export target_reassembly_policy="snort_windows"
export reassembly_options="flow-only-frag"
export target_os="windows_10"

#### snort_bsd

export target_ipv4_addr="192.168.10.66"
export target_ipv6_addr="fe80::200:ff:fe02:66"
export target_reassembly_policy="snort_bsd"
export reassembly_options="flow-only-frag"
export target_os="freebsd_13"

#### snort_bsd_right

export target_ipv4_addr="192.168.10.67"
export target_ipv6_addr="fe80::200:ff:fe02:67"
export target_reassembly_policy="snort_bsd_right"
export reassembly_options="flow-only-frag"

#### snort_solaris

export target_ipv4_addr="192.168.10.68"
export target_ipv6_addr="fe80::200:ff:fe02:68"
export target_reassembly_policy="snort_solaris"
export reassembly_options="flow-only-frag"
export target_os="solaris_11.2"

#### snort_last

export target_ipv4_addr="192.168.10.69"
export target_ipv6_addr="fe80::200:ff:fe02:69"
export target_reassembly_policy="snort_last"
export reassembly_options="flow-only-frag"

#### snort_first

export target_ipv4_addr="192.168.10.70"
export target_ipv6_addr="fe80::200:ff:fe02:70"
export target_reassembly_policy="snort_first"
export reassembly_options="flow-only-frag"

#### snort_default

export target_ipv4_addr="192.168.10.71"
export target_ipv6_addr="fe80::200:ff:fe02:71"
export target_reassembly_policy="snort_default"
export reassembly_options="flow-only-frag"

## TCP

### flow-only-stream rule file

#### snort_linux

export target_ipv4_addr="192.168.10.64"
export target_reassembly_policy="snort_linux"
export reassembly_options="flow-only-stream"
export target_os="debian_12"

#### snort_windows

export target_ipv4_addr="192.168.10.65"
export target_reassembly_policy="snort_windows"
export reassembly_options="flow-only-stream"

#### snort_bsd

export target_ipv4_addr="192.168.10.66"
export target_reassembly_policy="snort_bsd"
export reassembly_options="flow-only-stream"
export target_os="freebsd_13"

#### snort_bsd_right

export target_ipv4_addr="192.168.10.67"
export target_reassembly_policy="snort_bsd_right"
export reassembly_options="flow-only-stream"

#### snort_solaris

export target_ipv4_addr="192.168.10.68"
export target_reassembly_policy="snort_solaris"
export reassembly_options="flow-only-stream"
export target_os="solaris_11.2"

#### snort_last

export target_ipv4_addr="192.168.10.69"
export target_reassembly_policy="snort_last"
export reassembly_options="flow-only-stream"

#### snort_first

export target_ipv4_addr="192.168.10.70"
export target_reassembly_policy="snort_first"
export reassembly_options="flow-only-stream"
export target_os="windows_10"

#### snort_default

export target_ipv4_addr="192.168.10.71"
export target_reassembly_policy="snort_default"
export reassembly_options="flow-only-stream"

#### snort_old_linux

export target_ipv4_addr="192.168.10.72"
export target_reassembly_policy="snort_old_linux"
export reassembly_options="flow-only-stream"

#### snort_macos

export target_ipv4_addr="192.168.10.73"
export target_reassembly_policy="snort_macos"
export reassembly_options="flow-only-stream"

#### snort_irix

export target_ipv4_addr="192.168.10.74"
export target_reassembly_policy="snort_irix"
export reassembly_options="flow-only-stream"

#### snort_hpux10

export target_ipv4_addr="192.168.10.75"
export target_reassembly_policy="snort_hpux10"
export reassembly_options="flow-only-stream"

#### snort_hpux11

export target_ipv4_addr="192.168.10.76"
export target_reassembly_policy="snort_hpux11"
export reassembly_options="flow-only-stream"


#### snort_windows2k3

export target_ipv4_addr="192.168.10.77"
export target_reassembly_policy="snort_windows2k3"
export reassembly_options="flow-only-stream"

#### snort_vista

export target_ipv4_addr="192.168.10.78"
export target_reassembly_policy="snort_vista"
export reassembly_options="flow-only-stream"

#### snort_proxy

export target_ipv4_addr="192.168.10.79"
export target_reassembly_policy="snort_proxy"
export reassembly_options="flow-only-stream"
