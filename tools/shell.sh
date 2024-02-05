#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "You must be a root user to run this script, please run sudo ./install.sh" 2>&1
  exit 1
fi

#Install custom shell and color scheme
builddir=$(pwd)
pip3 install --user powerline-status
sudo apt install -y fonts-powerline

# Install Patched Font
mkdir ~/.fonts
sudo cp -a fonts/. ~/.fonts/
fc-cache -vf ~/.fonts/

# Install ZSH
cd ~
sudo apt install -y git-core zsh curl
sh -c "$(curl -fsSL https://raw.github.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"

#Copy theme to ZSH folder
cp ~/linux-utils/tools/matteleo.zsh-theme ~/.oh-my-zsh/themes/

cp .zshrc ~/.zshrc

chsh -s $(which zsh)
