#!/bin/bash

# install packet manager
git clone https://github.com/tmux-plugins/tpm ~/.tmux/plugins/tpm

# copy config file
mkdir /home/$USER/.config/tmux
cp /home/$USER/linutils/src/utils/tmux.conf /home/$USER/.config/tmux/tmux.conf

# copy script to launch tmux sessions for coding
cp /home/$USER/linutils/src/utils/code.sh /home/$USER/
