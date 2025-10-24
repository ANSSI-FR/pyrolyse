PYROLYSE
====

PYROLYSE is an audit tool that exhaustively tests and describes the overlap-related reassembly policies of various IPv4, IPv6, and TCP implementation types. 
It's composed of four main steps: the overlap test case generation, the stack testing, the reassembly policy extraction, and the reassembly policy analysis.
Currently, PYROLYSE generates exhaustive overlap test cases for up to 3 overlapping chunks, and we tested seven stack types: general-purpose OSes (e.g., Windows, Linux, FreeBSD), embedded/IoT stacks (e.g., lwIP, picoTCP), DPDK-compatible frameworks (Seastar), unikernel (mirage-tcpip), NIC, ICS, and NIDSes (Snort, Suricata, and Zeek). 
Check our paper for more details on the PYROLYSE pipeline and results [https://arxiv.org/pdf/2508.00735](https://arxiv.org/pdf/2508.00735). 
The obtained stack policies can be found at [https://nextcloud.centralesupelec.fr/s/jQYZsBnjnkCpbLo](https://nextcloud.centralesupelec.fr/s/jQYZsBnjnkCpbLo).

# Installation

Note: these commands are specific to the Fedora distribution, please change them if you use another OS.


## VM-related tools

### VirtualBox

    $ sudo dnf install Virtualbox-7.2.2-1
    $ sudo dnf install virtualbox-guest-additions.7.1.8-1

#### Add new networks (VirtualBox only)

You need to add as many virtual network as private network you want to use at the same time. 
As default, vboxnet2 is used; so you need to run twice the following command, if you don't have any vboxnet interface.

    $ VBoxManage hostonlyif create

### Vagrant

    $ sudo dnf install vagrant-2.4.9

#### Plugins 

    $ vagrant plugin install winrm # for Windows OS testing 
    $ vagrant plugin install vagrant-reload


## SparQ

At the root of the pyrolyse project:

    $ mkdir -p "${PYROLYSE_PATH}/sparq_dir"
    $ cd "${PYROLYSE_PATH}/sparq_dir"
    $ git clone https://github.com/dwolter/SparQ.git
    $ mv "${PYROLYSE_PATH}/sparq_dir/SparQ" "${PYROLYSE_PATH}/sparq_dir/SparQ_${HOSTNAME}"
    $ cd "${PYROLYSE_PATH}/sparq_dir/SparQ_${HOSTNAME}"
    $ git checkout f731136f76fba0225abdd457616fbfdcae83c00e
    $ autoreconf -fi
    $ ./configure
    $ make -j 6
    $ cd "${PYROLYSE_PATH}"


### with submodule

At the root of the pyrolyse project:

    $ cd "${PYROLYSE_PATH}/sparq/SparQ"
    $ git checkout 6dcf5c3
    $ autoreconf -fi
    $ ./configure
    $ make -j 6



## Rust toolchain

    $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

See [https://rustup.rs/](https://rustup.rs/) for more details.

Add the path to the cargo executable in your PATH environment variable. Its usual location is ~/.cargo/bin/.

WARNING: it is strongly advised to not do this as root.


## Rust tools

At the root of the pyrolyse project:

    $ cd "${PYROLYSE_PATH}/rust_code/reassembly_test_pipeline"
    $ cargo build
    $ cd "${PYROLYSE_PATH}"


## Command files in ws

Full commands are located in the ./ws directory.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Related publications

Check our two papers

## RAID'25

* Overlapping IPv4, IPv6, and TCP data: exploring errors, test case context, and multiple overlaps inside network stacks and NIDSes with PYROLYSE
* Lucas Aubard, Johan Mazel, Gilles Guette, Pierre Chifflier 
* DOI:10.1109/RAID67961.2025.00054
* [https://arxiv.org/pdf/2508.00735](https://arxiv.org/pdf/2508.00735)

```
@inproceedings{aubard2025raid,
  title={{Overlapping IPv4, IPv6, and TCP data: exploring errors, test case context and multiple overlaps inside network stacks and NIDSes with PYROLYSE}},
  author={Aubard, Lucas and Mazel, Johan and Guette, Gilles and Chifflier, Pierre},
  booktitle={RAID},
  year={2025}
}
```

## [DIMVA'25](https://dl.acm.org/doi/10.1007/978-3-031-97623-0_13)

We also used part of the PYROLYSE pipeline described in the RAID paper to obtain the results of this paper

* Overlapping data in network protocols: bridging OS and NIDS reassembly gap
* Lucas Aubard, Johan Mazel, Gilles Guette, Pierre Chifflier
* [https://arxiv.org/pdf/2504.21618](https://arxiv.org/pdf/2504.21618)

```
@inproceedings{aubard2025dimva,
  title={{Overlapping data in network protocols: bridging OS and NIDS reassembly gap}},
  author={Aubard, Lucas and Mazel, Johan and Guette, Gilles and Chifflier, Pierre},
  booktitle={International Conference on Detection of Intrusions and Malware, and Vulnerability Assessment},
  pages={216--236},
  year={2025},
  organization={Springer}
}
```