#!/bin/bash

# Workspace 1: Browser
hyprctl dispatch exec "[workspace 1 silent] flatpak run app.zen_browser.zen"
# Workspace 2: Sessionized terminal with nvim, lazygit
hyprctl dispatch exec "[workspace 2 silent] alacritty -e ./code.sh"

notify-send "Ready to code"
