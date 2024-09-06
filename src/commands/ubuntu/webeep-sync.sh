#!/bin/bash

# Install webeep
wget -O webeep.deb "https://github.com/toto04/webeep-sync/releases/latest/download/webeep-sync-debian.deb"
sudo apt install ./webeep.deb -y
rm webeep.deb
