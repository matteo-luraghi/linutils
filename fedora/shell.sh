#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
	echo "You must be a root user to run this script, please run sudo ./shell.sh" 2>&1
	exit 1
fi

# Install custom shell and color scheme
pip3 install --user powerline-status

# Install Patched Font
mkdir ~/.fonts
sudo cp -a ~/linux-utils/tools/fonts/. ~/.fonts/
fc-cache -vf ~/.fonts/

# Install ZSH
cd ~
dnf install -y git-core zsh curl
sh -c "$(curl -fsSL https://raw.github.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"

# Install plug-ins
cd ~/.oh-my-zsh/custom/plugins
git clone https://github.com/zsh-users/zsh-syntax-highlighting
git clone https://github.com/zsh-users/zsh-autosuggestions

# Color Theme
dconf load /org/gnome/terminal/legacy/profiles:/:b1dcc9dd-5262-4d8d-a863-c897e6d979b9/ <~/linutils/tools/color_theme.dconf

#Copy theme to ZSH folder
cp ~/linutils/tools/matteleo.zsh-theme ~/.oh-my-zsh/themes/

cp ~/linutils/tools/.zshrc ~/.zshrc

chsh -s $(which zsh)
