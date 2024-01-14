#!/bin/bash

builddir=$(pwd)
git clone https://github.com/pixegami/terminal-profile.git
./terminal-profile/install_powerline.sh
cd $builddir
./terminal-profile/install_terminal.sh
cd $builddir
./terminal-profile/install_profile.sh
cd $builddir
rm -r terminal-profile
