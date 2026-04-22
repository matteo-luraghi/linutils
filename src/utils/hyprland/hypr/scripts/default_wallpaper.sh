#!/bin/bash

# define the exact path to the wallpaper
TARGET="$HOME/Pictures/wallpapers/wallpaper.jpg"

# check if swww-daemon is running, start it if not
if ! pgrep -x "swww-daemon" > /dev/null; then
    swww-daemon &
    sleep 1
fi

# check if the specific file exists before trying to apply it
if [ ! -f "$TARGET" ]; then
    notify-send "Wallpaper Script" "Could not find $TARGET"
    exit 1
fi

# apply the specific wallpaper
swww img "$TARGET" --transition-type wipe --transition-angle 30 --transition-step 90
