

# compilation


https://redmine.openinfosecfoundation.org/projects/suricata/wiki/Ubuntu_Installation_from_GIT


sudo apt-get -y install build-essential libpcap-dev libpcre3-dev libnet1-dev zlib1g-dev luajit hwloc libdnet-dev libdumbnet-dev bison flex liblzma-dev openssl libssl-dev pkg-config libhwloc-dev cmake cpputest libsqlite3-dev uuid-dev libcmocka-dev libnetfilter-queue-dev libmnl-dev autotools-dev libluajit-5.1-dev libunwind-dev libfl-dev libpcre2-dev


## daq


git clone https://github.com/snort3/libdaq.git

cd libdaq

./bootstrap

./configure \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/snort_base/install/install_libdaq

make -j 6

make install

cd ..




## git clone


rm -rf snort3


git clone https://github.com/snort3/snort3.git


cp -r snort3 snort3_save





## init


cd ..

rm -rf snort3

cp -r snort3_save snort3

cd snort3





## 3.0.3-6



git checkout 537e071ae6888cc8b645aa5270033cb85dbea32f

./configure_cmake.sh \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/snort_base/install/install_3.0.3-6 \
--with-daq-includes=$PYROLYSE_PATH/target/midpoint/ids_code/snort_base/install/install_libdaq/include \
--with-daq-libraries=$PYROLYSE_PATH/target/midpoint/ids_code/snort_base/install/install_libdaq/lib

cd build

make -j 6

make install

cd ..




DAQ_LIBRARIES DAQ_INCLUDE_DIR


./configure --with-daq-includes=<inc dir>
            --with-daq-libraries=<lib dir>


## 3.1.58.0

git checkout fc35a68d840cfe71325dd154374ef6494108b88c

./configure_cmake.sh \
--prefix="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.1.58.0" \
--with-daq-includes="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/include" \
--with-daq-libraries="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib"

cd build

make -j 6

make install



## 3.1.81.0

git checkout be0977a

./configure_cmake.sh \
--prefix="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.1.81.0" \
--with-daq-includes="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/include" \
--with-daq-libraries="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib"

cd build

make -j 6

make install



## 3.6.3.0

git checkout 46545be8b3588661a045b7010960146bce304540

./configure_cmake.sh \
--prefix="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.6.3.0" \
--with-daq-includes="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/include" \
--with-daq-libraries="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib"

cd build

make -j 6

make install

## 3.7.1.0

git checkout 6a11279883a8584e06ad9ab2df162c639961cd61

./configure_cmake.sh \
--prefix="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.7.1.0" \
--with-daq-includes="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/include" \
--with-daq-libraries="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib"

cd build

make -j 6

make install


## debug

(cf https://github.com/snort3/snort3/blob/a192c18a848f4df97bf00e16a97c2027ec65e61b/doc/user/trace.txt)

git checkout fc35a68d840cfe71325dd154374ef6494108b88c

./configure_cmake.sh \
--prefix=$PYROLYSE_PATH/target/midpoint/ids_code/snort_base/install/install_3.1.58.0_debug \
--with-daq-includes=$PYROLYSE_PATH/target/midpoint/ids_code/snort_base/install/install_libdaq/include \
--with-daq-libraries=$PYROLYSE_PATH/target/midpoint/ids_code/snort_base/install/install_libdaq/lib \
--enable-debug-msgs

cd build

make -j 6

make install







