#!/bin/sh
echoerr() { echo "$@" 1>&2; }
echoerr "test crushed" && exit 1
