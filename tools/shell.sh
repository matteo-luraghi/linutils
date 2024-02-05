#!/bin/bash

#Install custom shell and color scheme
builddir=$(pwd)
pip3 install --user powerline-status
sudo apt install -y fonts-powerline

# Install Patched Font
mkdir ~/.fonts
sudo cp -a fonts/. ~/.fonts/
fc-cache -vf ~/.fonts/

# Install ZSH
sudo apt install -y git-core zsh curl
sh -c "$(curl -fsSL https://raw.github.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"

chsh -s $(which zsh)
