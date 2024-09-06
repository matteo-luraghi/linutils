#!/bin/bash

# Download webeep
wget -O webeep.rpm "https://github.com/toto04/webeep-sync/releases/latest/download/webeep-sync-redhat.rpm"

# Install webeep
sudo dnf install webeep.rpm -y

# Remove installation file
rm webeep.rpm
