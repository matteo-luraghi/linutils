#!/bin/bash

# Workspace 1: Terminal
hyprctl dispatch exec "[workspace 1 silent] alacritty"
# Workspace 2: Browser
hyprctl dispatch exec "[workspace 2 silent] brave-browser --ozone-platform-hint=auto --enable-features=WaylandWindowDecorations"
# Workspace 3: Sessionized terminal with nvim, lazygit
hyprctl dispatch exec "[workspace 3 silent] alacritty -e ./code.sh"

notify-send "Ready to code"
