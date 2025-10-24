# compilation


https://redmine.openinfosecfoundation.org/projects/suricata/wiki/Ubuntu_Installation_from_GIT


sudo apt-get -y install libpcre3 libpcre3-dbg libpcre3-dev \
build-essential autoconf automake libtool libpcap-dev libnet1-dev \
libyaml-0-2 libyaml-dev pkg-config zlib1g zlib1g-dev libcap-ng-dev libcap-ng0 \
make libmagic-dev libjansson-dev rustc cargo jq git-core cbindgen





## initial clone



cd $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/


rm -rf suricata

rm -rf suricata suricata_save



git clone https://github.com/OISF/suricata.git


cp -r suricata suricata_save


cd suricata




## init


cd ..

rm -rf suricata

cp -r suricata_save suricata

cd suricata



## save default conf


cp suricata.yaml.in ../suricata.yaml






## 2.0.0



git checkout bc70fc0f793ffea24fa8c4fe94ea1b8e54c43b61

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_2.0.0

make -j 6

make install



mkdir -p $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_2.0.0/etc/suricata

cp suricata.yaml $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_2.0.0/etc/suricata



## 2.0.11 



git checkout c80420a

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_2.0.11

make -j 6

make install



mkdir -p $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_2.0.11/etc/suricata

cp suricata.yaml $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_2.0.11/etc/suricata





## 3.0.0



git checkout f9faf990fb08c1a8df5694d7156300f909422f58

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_3.0.0

make -j 6

make install



mkdir -p $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_3.0.0/etc/suricata

cp suricata.yaml $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_3.0.0/etc/suricata




## 3.2.5



git checkout 2fa58ce

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_3.2.5

make -j 6

make install
make install-conf
make install-data



mkdir -p $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_3.2.5/etc/suricata

cp suricata.yaml $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_3.2.5/etc/suricata




## 4.0.0



git checkout b8428378ac6fb2365337ae765e19dfc0f4548e4a

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_4.0.0

make -j 6

make install
make install-conf
make install-data



mkdir -p $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_4.0.0/etc/suricata

cp suricata.yaml $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_4.0.0/etc/suricata





## 4.1.10


sudo apt install autoconf automake libtool libyaml-dev libhtp-dev -y


git checkout 7405cd13731a0108fa3dfa34ee1e8caf30566bb9

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_4.1.10

make -j 6

make install
make install-conf
make install-data



mkdir -p $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_4.0.0/etc/suricata

cp suricata.yaml $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_4.0.0/etc/suricata




## 5.0.0


git checkout 697410cbbfe351afd307424bab5590955a1b95ae

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_5.0.0

make -j 6

make install
make install-conf
make install-data


mkdir -p $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_5.0.0/etc/suricata

cp suricata.yaml $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_5.0.0/etc/suricata




## 5.0.10


git checkout 252c2d0

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_5.0.10

make -j 6

make install
make install-conf
make install-data


mkdir -p $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_5.0.10/etc/suricata

cp suricata.yaml $PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_5.0.10/etc/suricata



## 6.0.0


TODO: check make install-conf et make install-data ici


git checkout 5219691f45ab6c6fb3c3970550dd670c73dbcfa6

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_6.0.0

make -j 6

make install




## 6.0.1


git checkout e860b9eee96cb112a45151237d4b23fd0a9efb22

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_6.0.1/

make -j 6

make install && make install-conf


### debug (https://redmine.openinfosecfoundation.org/issues/3233)

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_6.0.1_debug/usr/ \
--localstatedir=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_6.0.1_debug/var/ \
--sysconfdir=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_6.0.1_debug/etc/ \
--enable-debug

make clean && make

export SC_LOG_LEVEL=Debug



## 6.0.15


git checkout d20c933

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_6.0.15/

make -j 6

make install && make install-conf



## 7.0.0-rc1


git checkout d9e6301af2e86c9ab1686697928c753cc0c18285

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_7.0.0_rc1

make -j 6

make install-full



## 7.0.2


git checkout d07e20c

git clone https://github.com/OISF/libhtp.git -b 0.5.x

./autogen.sh

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/suricata_base/install/install_7.0.2

make -j 6

make install-full



