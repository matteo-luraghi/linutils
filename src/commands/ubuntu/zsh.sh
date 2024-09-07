#!/bin/bash

# Install custom shell and color scheme
pip3 install --user powerline-status
apt install -y fonts-powerline

# Install Patched Font
mkdir ~/.fonts
sudo cp -a ~/linutils/src/utils/fonts/. ~/.fonts/
fc-cache -vf ~/.fonts/

# Install ZSH
cd ~
apt install -y git-core zsh curl
sh -c "$(curl -fsSL https://raw.github.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"

# Install plug-ins
(cd ~/.oh-my-zsh/custom/plugins && git clone https://github.com/zsh-users/zsh-syntax-highlighting)
(cd ~/.oh-my-zsh/custom/plugins && git clone https://github.com/zsh-users/zsh-autosuggestions)

# Color Theme
dconf load /org/gnome/terminal/legacy/profiles:/:b1dcc9dd-5262-4d8d-a863-c897e6d979b9/ < ~/linutils/src/utils/color_theme.dconf

# Copy theme to ZSH folder
cp ~/linutils/src/utils/matteleo.zsh-theme ~/.oh-my-zsh/themes/

cp ~/linutils/src/utils.zshrc ~/.zshrc

# Change shell for the current user
sudo chsh -s $(which zsh) $USER
