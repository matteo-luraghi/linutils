#!/bin/bash

sudo dnf install git-core zsh curl python3-pip -y

# Install custom shell and color scheme
pip3 install --user powerline-status

# Install Patched Font
mkdir /home/$USER/.fonts
sudo cp -a /home/$USER/linutils/src/utils/fonts/. /home/$USER/.fonts/
fc-cache -vf ~/.fonts/

# Install ZSH
cd /home/$USER
sh -c "$(curl -fsSL https://raw.github.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"

# Install plug-ins
cd /home/$USER/.oh-my-zsh/custom/plugins
git clone https://github.com/zsh-users/zsh-syntax-highlighting
git clone https://github.com/zsh-users/zsh-autosuggestions

# Color Theme
dconf load /org/gnome/terminal/legacy/profiles:/:b1dcc9dd-5262-4d8d-a863-c897e6d979b9/ < "/home/$USER/linutils/src/utils/color_theme.dconf"

#Copy theme to ZSH folder
cp /home/$USER/linutils/src/utils/matteleo.zsh-theme /home/$USER/.oh-my-zsh/themes/

cp /home/$USER/linutils/src/utils/.zshrc /home/$USER/.zshrc

# Change shell for the current user
sudo chsh -s $(which zsh) $USER
