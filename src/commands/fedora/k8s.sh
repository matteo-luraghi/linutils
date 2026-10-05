#!/bin/bash

# tool to change context easily
sudo dnf copr enable audron/kubectx
sudo dnf install kubectx

# destkop app to manage clusters
flatpak install io.kinvolk.Headlamp

sudo dnf install kubectl -y
