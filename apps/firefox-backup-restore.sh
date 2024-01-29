#!/bin/bash

#Restore firefox backup
gpg $builddir/linux-installer/apps/firefox.backup.tar.bz2.gpg
tar -xf $builddir/linux-installer/apps/firefox.backup.tar.bz2
rm -r $builddir/linux-installer/apps/firefox.backup.tar.bz2
