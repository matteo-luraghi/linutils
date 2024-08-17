#!/bin/bash

# Restore firefox backup
gpg $builddir/linux-utils/tools/firefox.backup.tar.bz2.gpg
tar -xf $builddir/linux-utils/tools/firefox.backup.tar.bz2
rm -r $builddir/linux-utils/tools/firefox.backup.tar.bz2
