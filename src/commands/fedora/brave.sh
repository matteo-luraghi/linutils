#!/bin/bash

dnf install dnf-plugins-core
dnf config-manager addrepo --from-repofile=https://brave-browser-rpm-release.s3.brave.com/brave-browser.repo
yes | dnf install brave-browser
