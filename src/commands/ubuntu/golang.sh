#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "You must be a root user to run this script, please run sudo ./golang.sh" 2>&1
  exit 1
fi

# Install go
goVersion=go1.21.3.linux-amd64.tar.gz
wget https://go.dev/dl/$goVersion
chmod u+x $goVersion
tar -C /usr/local -xzf $goVersion
rm $goVersion
