#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "You must be a root user to run this script, please run sudo ./nala.sh" 2>&1
  exit 1
fi

# Install nala
apt install nala -y
