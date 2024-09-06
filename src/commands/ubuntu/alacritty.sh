#!/bin/bash

# Install alacritty
sudo add-apt-repository ppa:aslatter/ppa -y
sudo apt update -y
sudo apt install alacritty -y

# Setup alacritty
mkdir ~/.config/alacritty
cp ~/linutils/src/utils/alacritty.toml ~/.config/alacritty
