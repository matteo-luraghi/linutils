#Install docker
dockerVersion=docker-desktop-4.25.0-x86_64.rpm
dnf install dnf-plugins-core -y
dnf config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo
dnf install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y
wget https://desktop.docker.com/linux/main/amd64/$dockerVersion
chmod u+x $dockerVersion
dnf install ./$dockerVersion -y
rm $dockerVersion
