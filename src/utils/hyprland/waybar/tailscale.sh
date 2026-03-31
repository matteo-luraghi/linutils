#!/bin/bash

# Toggle logic
if [ "$1" == "toggle" ]; then
    if tailscale status | grep -q "Tailscale is stopped"; then
        tailscale up
    else
        tailscale down
    fi
    # Pause briefly to allow the connection state to change before reporting back
    sleep 1 
fi

# Status logic for Waybar
if tailscale status | grep -q "Tailscale is stopped"; then
    #  is the FontAwesome shield icon. You can swap this for a lock () if preferred.
    echo '{"text": "", "class": "off", "tooltip": "Tailscale: Disconnected"}'
else
    echo '{"text": "", "class": "on", "tooltip": "Tailscale: Connected"}'
fi
