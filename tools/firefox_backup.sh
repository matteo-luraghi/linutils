#!/bin/bash

username=$(id -u -n 1000)

# Backup firefox
cd /home/$username
tar -jcvf firefox-backup.tar.bz2 .mozilla
gpg -c firefox-backup.tar.bz2

rm firefox-backup.tar.bz2
mv firefox-backup.tar.bz2.gpg /home/$username/linutils/apps/
