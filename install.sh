#!/bin/sh

#
# This will install to the $HOME directory with a symlink add in /usr/local/bin

# if /usr/local/bin is not in the PATH then prompt the user to add it

REPO=smyleeface/RustyFileNinja

# USER=${USER:-$(id -u -n)}

INSTALL_DIR=$HOME
FILENINJA_INSTALL=$INSTALL_DIR/file-ninja
BIN_DIR=/usr/local/bin

# TODO: check os type
# TODO: get file version from Cargo.toml?
FILENINJA_VERSION=v0.0.2
FILENINJA_FILENAME=rusty_file_ninja_${FILENINJA_VERSION}_linux_x64.tar.gz
FILENINJA_RELEASE_TAR="https://github.com/${REPO}/releases/download/${FILENINJA_VERSION}/${FILENINJA_FILENAME}"

echo "Downloading file ninja: $INSTALL_TOOL"

echo Make file-ninja directory in the $INSTALL_DIR

if type "curl"; then
  sh -c "$(curl -fsSL $FILENINJA_RELEASE_TAR | tar -xz)"
elif type "wget"; then
  sh -c "$(wget $FILENINJA_RELEASE_TAR -O - | tar -xz)"
else
  printf "cannot download installer"
fi

export PATH=$FILENINJA_INSTALL:PATH

file-ninja --help