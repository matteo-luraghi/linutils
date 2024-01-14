#!/bin/bash

#Install Java
dnf install default-jdk -y
dnf install default-jre -y

#Install go
goVersion=go1.21.3.linux-amd64.tar.gz
wget https://go.dev/dl/$goVersion
chmod u+x $goVersion
tar -C /usr/local -xzf $goVersion
echo 'export PATH=$PATH:/usr/local/go/bin' >> .zshrc
rm $goVersion

#Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
