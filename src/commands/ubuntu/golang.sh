#!/bin/bash

# Install go
goVersion=go1.21.3.linux-amd64.tar.gz
wget https://go.dev/dl/$goVersion
chmod u+x $goVersion
tar -C /usr/local -xzf $goVersion
rm $goVersion
