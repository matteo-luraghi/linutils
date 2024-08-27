#!/bin/bash

# Backup firefox
cd ~
tar -jcvf firefox-backup.tar.bz2 .mozilla
gpg -c firefox-backup.tar.bz2

rm firefox-backup.tar.bz2
mv firefox-backup.tar.bz2.gpg ~/linutils/src/utils
