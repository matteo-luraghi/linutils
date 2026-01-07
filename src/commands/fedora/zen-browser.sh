#!/bin/bash

flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo

flatpak install flathub app.zen_browser.zen

xdg-settings set default-web-browser app.zen_browser.zen.desktop
