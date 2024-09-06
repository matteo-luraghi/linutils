#!/bin/bash

# Install lazygit
sudo dnf copr enable atim/lazygit -y
sudo dnf install lazygit -y

# Copy lazygit config
cp ~/linutils/src/utils/lazygit.yml ~/.config/lazygit/config.yml
