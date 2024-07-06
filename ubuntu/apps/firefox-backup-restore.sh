#!/bin/bash

#Restore firefox backup
gpg $builddir/linux-utils/apps/firefox.backup.tar.bz2.gpg
tar -xf $builddir/linux-utils/apps/firefox.backup.tar.bz2
rm -r $builddir/linux-utils/apps/firefox.backup.tar.bz2
