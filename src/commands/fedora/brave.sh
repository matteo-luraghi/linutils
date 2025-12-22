#!/bin/bash

sudo dnf install dnf-plugins-core
sudo dnf config-manager addrepo --from-repofile=https://brave-browser-rpm-release.s3.brave.com/brave-browser.repo
yes | sudo dnf install brave-browser
sudo mkdir /etc/brave/policies/managed/ -p
sudo cp /home/$USER/linutils/src/utils/brave-policies.json /etc/brave/policies/managed/
