#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
	echo "You must be a root user to run this script, please run sudo ./webeep.sh" 2>&1
	exit 1
fi

# Download webeep
wget -O webeep.rpm "https://github.com/toto04/webeep-sync/releases/latest/download/webeep-sync-redhat.rpm"

# Install webeep
dnf install webeep.rpm -y

# Remove installation file
rm webeep.rpm
