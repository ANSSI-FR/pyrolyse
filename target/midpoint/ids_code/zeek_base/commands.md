




# Release-related commits


https://github.com/zeek/zeek/tags


1.5.3: 2b6ad76bd53ff69eec5ebcc47f7f5b42558163ae

3.0.0: a5557586699d9a90aba70a7a0468549c400e9b61

3.0.1: ae4740fa265701f494df23b65af80822f3e26a13

4.0.0-rc1: 16396ee8990068a61112d23b0ec43f96d23c440c

5.2.0-rc1: 54b816f91efd20900ab35f6204ca7658aa741f69


# compilation


https://docs.zeek.org/en/master/install/install.html

### Ubuntu

sudo apt-get install cmake make gcc g++ flex bison libpcap-dev libssl-dev python3 python3-dev swig zlib1g-dev

### Fedora

sudo dnf install cmake make gcc g++ flex bison libpcap-devel openssl-devel python3 python3-devel swig zlib-devel


## initial clone


rm -rf zeek


git clone https://github.com/zeek/zeek.git

cp -r zeek zeek_save





## init


cd ..

rm -rf zeek

cp -r zeek_save zeek

cd zeek





## 1.5.3


sudo apt install autoconf automake libtool libcrypto++-dev


git checkout 2b6ad76bd53ff69eec5ebcc47f7f5b42558163ae

./autogen.sh

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_1.5.3

make -j 6

make install




configure error with libcrypto




## 2.0


sudo apt install libmagick++-dev libmagic-dev


git checkout 5ae95bfc49716211e3643497b5585d39c8b635d8

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_2.0

make -j 6

make install




compilation error in $PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/zeek/aux/broctl/aux/pysubnettree/SubnetTree.cc










## 2.4


git checkout af1a6634101324a6421049e12a652096be938546

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_2.4

make -j 6

make install








## 2.4.2


git checkout c921b3fe589923ff142058fe72cac4971924208c

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_2.4.2

make -j 6

make install



Error:
/usr/bin/ld: ../src/libbroccoli.so.5.1.0: undefined reference to `SSLv3_method'

https://github.com/zeek/broccoli

http://mailman.icsi.berkeley.edu/pipermail/zeek/2018-November/013777.html




## 2.5


git checkout 7b44974a58181df657bd1edbe734e923d68b9ecc

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_2.5

make -j 6

make install


http://mailman.icsi.berkeley.edu/pipermail/zeek/2018-June/013314.html



## 2.5.5


git checkout 697aff2c0d569a8db1ca6b990a10838665a002c8

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_2.5.5

make -j 6

make install



error:
$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/zeek/src/file_analysis/analyzer/x509/X509.cc: In static member function ‘static RecordVal* file_analysis::X509::ParseCertificate(file_analysis::X509Val*, const char*)’:






## 2.6


git checkout f78c697fa0529ae4f9baed134f2e3caef6023d83

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_2.6

make -j 6

make install




## 2.6.4


git checkout 3b5a9f88ece1d274edee897837e280ef751bde94

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_2.6.4

make -j 6

make install








## 3.0.0



git checkout a5557586699d9a90aba70a7a0468549c400e9b61

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_3.0.0

make -j 6

make install




## 3.0.1


git checkout ae4740fa265701f494df23b65af80822f3e26a13

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_3.0.1

make -j 6

make install




## 3.1.0


git checkout cd75d21e24610ec9a594e1971dbb739ecdf4cc64

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_3.1.0

make -j 6

make install





## 3.2.0


git checkout 8e79c8dcd14b185726c5f7be9d31bb8e93686d46

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_3.2.0

make -j 6

make install





## 4.0.0-rc1


git checkout 16396ee8990068a61112d23b0ec43f96d23c440c

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install/install_4.0.0-rc1

make -j 6

make install




## 4.0.9


git checkout 3b00f0e

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install_$HOSTNAME/install_4.0.9

make -j 4

make install




# 5.2.0-rc1


git checkout 54b816f91efd20900ab35f6204ca7658aa741f69

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install_$HOSTNAME/install_5.2.0-rc1

make -j 6

make install




# 5.2.2


git checkout a6f825b

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install_$HOSTNAME/install_5.2.2

make -j 6

make install




# 6.2.0


git checkout e90c6e4

git submodule update --recursive --init

./configure --prefix=$PYROLYSE_PATH/target/midpoint/ids_code/zeek_base/install_$HOSTNAME/install_6.2.0

make -j 6

make install





