

# attack design

* Fragment IP payload and overlap on TCP source/destination port
* Firewall need to reassemble to check if port is authorized
* If firewall reassemble IP fragments differently than target OS, it will let packet go through but they will reach different port on target.

##

similar to https://www.monkey.org/~dugsong/fragroute/


# test methodology

* topology: base -> fw(s) -> target(s)
* firewall reassembly packet extraction
  - test case length
    + maximum length of test case data (w/o scenario) : 5 chunks for (O,O,O) => we need at least 5 bytes (more if we want to use scenarii)
    => UDP is too short
    => TCP
  - targeted fields by overlap : dst port and sequence number
* how to obtain post-firewall reassembly data
  - single firewall and single target: TCP mangle: dst port -> payload AND dst port = 7
  - single firewall and multiple targets (one for each possible dst port)
* 



## TCP mangling to copy dst port to payload

https://wiki.nftables.org/wiki-nftables/index.php/Mangling_packet_headers

https://www.netfilter.org/documentation/HOWTO/netfilter-hacking-HOWTO-4.html#ss4.4

https://netfilter.org/projects/libnetfilter_queue/doxygen/html/group__tcp.html

http://charette.no-ip.com:81/programming/doxygen/netfilter/group__tcp.html







## how to extract firewall reassembly

### Multiple fw with each a single couple src-dst port



# counter-measures

## detect fragmentation with small packets as bad

## martian packet in Linux kernel ?

## invalid state in Linux kernel

https://linux.die.net/man/8/iptables

