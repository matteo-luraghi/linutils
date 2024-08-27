#!/bin/bash

# Restore firefox backup
gpg $builddir/linutils/src/utils/firefox.backup.tar.bz2.gpg
tar -xf $builddir/linutils/src/utils/firefox.backup.tar.bz2
rm -r $builddir/linutils/src/utils/firefox.backup.tar.bz2
