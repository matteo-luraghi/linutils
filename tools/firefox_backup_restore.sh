#!/bin/bash

# Restore firefox backup
gpg $builddir/linutils/tools/firefox.backup.tar.bz2.gpg
tar -xf $builddir/linutils/tools/firefox.backup.tar.bz2
rm -r $builddir/linutils/tools/firefox.backup.tar.bz2
