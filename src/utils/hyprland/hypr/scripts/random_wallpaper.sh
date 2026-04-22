#!/bin/bash

# directory containing your wallpapers
DIR="$HOME/Pictures/wallpapers"

# check if swww-daemon is running, start it if not
if ! pgrep -x "swww-daemon" > /dev/null; then
    swww-daemon &
    sleep 1
fi

# find images, explicitly EXCLUDING wallpaper.jpg, shuffle, and pick one
RANDOM_PIC=$(find "$DIR" -type f \( -iname \*.jpg -o -iname \*.png -o -iname \*.webp \) ! -name "wallpaper.jpg" -print0 | shuf -z -n 1 | tr -d '\0')

# check if a picture was actually found
if [ -z "$RANDOM_PIC" ]; then
    notify-send "Wallpaper Script" "No valid images found in $DIR"
    exit 1
fi

# apply the random wallpaper
swww img "$RANDOM_PIC" --transition-type wipe --transition-angle 30 --transition-step 90
