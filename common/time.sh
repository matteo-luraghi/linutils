#!/bin/bash

#Prevents time error when using dual boot with windows
sudo timedatectl set-local-rtc 1 --adjust-system-clock
