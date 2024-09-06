#!/bin/bash

# Install alacritty
sudo dnf install alacritty -y

# Setup alacritty
mkdir /home/$USER/.config/alacritty
cp /home/$USER/linutils/src/utils/alacritty.toml /home/$USER/.config/alacritty
