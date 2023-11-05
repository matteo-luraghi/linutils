#!/bin/bash

# Check if Script is Run as Root
if [[ $EUID -ne 0 ]]; then
  echo "You must be a root user to run this script, please run sudo ./install.sh" 2>&1
  exit 1
fi

username=$(id -u -n 1000)

apt update
apt upgrade -y

apt install nala -y

#Install basic packages
nala install build-essential python3 btop ffmpeg firefox fzf tldr neofetch tree ca-certificates curl gnupg cowsay -y

# Install ZSH and plugins
nala install -y git-core zsh curl
sh -c "$(curl -fsSL https://raw.github.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
(cd ~/.oh-my-zsh/custom/plugins && git clone https://github.com/zsh-users/zsh-syntax-highlighting)
(cd ~/.oh-my-zsh/custom/plugins && git clone https://github.com/zsh-users/zsh-autosuggestions)
cp configs/.zshrc ~/.zshrc

#Install Java
nala install default-jdk -y
nala install default-jre -y

#Install go
goVersion=go1.21.3.linux-amd64.tar.gz
wget https://go.dev/dl/$goVersion
chmod u+x $goVersion
tar -C /usr/local -xzf $goVersion
echo 'export PATH=$PATH:/usr/local/go/bin' >> .zshrc
rm $goVersion

#Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#Install and configure nvim
snap install nvim
git clone https://github.com/AstroNvim/AstroNvim ~/.config/nvim
git clone https://github.com/matteo-luraghi/astro-nvimsetup ~/.config/nvim/lua/user

#Install docker
dockerVersion=docker-desktop-4.25.0-amd64.deb
modprobe kvm
sudo install -m 0755 -d /etc/apt/keyrings -y
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg
echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
nala update
nala install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y
wget https://desktop.docker.com/linux/main/amd64/$dockerVersion
chmod u+x $dockerVersion
nala install ./$dockerVersion
rm $dockerVersion

#Move wallpaper
mkdir -p /home/$username/Pictures/Wallpapers
cp wallpaper.jpg /home/$username/Pictures/

#Update and reboot
nala update
nala upgrade -y
chsh -s $(which zsh)
