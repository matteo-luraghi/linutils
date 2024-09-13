#!/bin/bash

# Install lazygit
sudo dnf copr enable atim/lazygit -y
sudo dnf install lazygit -y

mkdir -p /home/$USER/.config/lazygit

# Copy lazygit config
cp /home/$USER/linutils/src/utils/lazygit.yml /home/$USER/.config/lazygit/config.yml
