#!/bin/bash

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
nala install ./$dockerVersion -y
rm $dockerVersion
