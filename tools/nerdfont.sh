#!/bin/bash

# Install nerdfont for nvim
wget -P ~/.local/share/fonts https://github.com/ryanoasis/nerd-fonts/releases/download/v3.2.1/DejaVuSansMono.zip \
&& cd ~/.local/share/fonts \
&& unzip DejaVuSansMono.zip \
&& rm DejaVuSansMono.zip \
&& fc-cache -fv

