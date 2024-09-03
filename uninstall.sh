#!/bin/sh

# This installer will download the application files to the $HOME directory
OS_TYPE=$(uname -s)
if [ "$OS_TYPE" != "Linux" ]; then
    echo "!!! OS currently not supported"
    exit 1
fi

REPO=smyleeface/RustyFileNinja
INSTALL_DIR=$HOME
FILENINJA_INSTALL=$INSTALL_DIR/file-ninja
FILENINJA_VERSION=v0.0.3
FILENINJA_FILENAME=rusty_file_ninja_${FILENINJA_VERSION}_linux_x64.tar.gz
FILENINJA_RELEASE_TAR="https://github.com/${REPO}/releases/download/${FILENINJA_VERSION}/${FILENINJA_FILENAME}"

printf "\nThis uninstaller will remove file-ninja from: %s" "$FILENINJA_INSTALL"
printf "\nEnter Y to continue, or any other key to exit. "
read -r user_input
if [ ! "$user_input" = "Y" ] && [ ! "$user_input" = "y" ]; then
    printf "\nExiting uninstaller."
    printf "\n"
    printf "\n"
    exit 0
fi

rm -fdr "$FILENINJA_INSTALL"

printf "\n"
printf "############################## FILE-NINJA ##############################"
printf "\n#"
printf "\n# Deleted executable located at: %s" "$FILENINJA_INSTALL"
printf "\n#"
printf "\n# Remove file-ninja from your PATH, remove"
printf "\n# the following from your ~/.bashrc or ~/.zshrc:"
printf "\n#"
printf "\n#   export PATH=%s/release:\$PATH" "$FILENINJA_INSTALL"
printf "\n#"
printf "\n# Then restart your terminal or source ~/.bashrc or ~/.zshrc file."
printf "\n#"
printf "\n#"
printf "########################################################################"
printf "\n"
