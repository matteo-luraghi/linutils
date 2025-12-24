#!/bin/bash

# Install alacritty
sudo dnf install alacritty -y

# Setup alacritty
mkdir -p /home/$USER/.config/alacritty
cp /home/$USER/linutils/src/utils/alacritty.toml /home/$USER/.config/alacritty
cp /home/$USER/linutils/src/utils/fonts/RobotoMono /home/$USER/.local/share/fonts
fc-cache -fv
