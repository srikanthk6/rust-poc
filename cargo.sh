#Install RUST
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

sudo apt update 
sudo apt install libssl-dev git cargo pkg-config -y

git clone https://github.com/srikanthk6/rust-poc.git
cd rust-poc

#Run below commented lines if <libsys openssl error>
# git clone git://git.openssl.org/openssl.git
# cd openssl
# ./config --openssldir=/usr/local/ssl
# make
# sudo make install
# export OPENSSL_DIR="/usr/local/ssl"

#Build for prod release
cargo build --release
#Run the package
cargo run 