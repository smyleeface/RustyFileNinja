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

printf "\nThis installer will install file-ninja to the %s directory." "$INSTALL_DIR"
printf "\nEnter Y to continue, or any other key to exit. "
read -r user_input
if [ ! "$user_input" = "Y" ] && [ ! "$user_input" = "y" ]; then
    printf "\nExiting installer."
    printf "\n"
    printf "\n"
    exit 0
fi

if type "curl" > /dev/null 2>&1; then
  COMMAND="$(curl -fsSL $FILENINJA_RELEASE_TAR | tar -xz)"
elif type "wget" > /dev/null 2>&1; then
  COMMAND="$(wget $FILENINJA_RELEASE_TAR -qO - | tar -xz)"
else
  printf "!!! Cannot download application. Please install \`curl\` or \`wget\`."
  exit 1
fi

printf "\n"
printf "############################## FILE-NINJA ##############################"

printf "\n#"
printf "\n# Downloading and extracting: %s" "$FILENINJA_RELEASE_TAR"
sh -c "$COMMAND"

export "PATH=$FILENINJA_INSTALL/release:$PATH"

if ! file-ninja --help > /dev/null 2>&1; then
  printf "!!! There was an error running file-ninja"
  file-ninja --help
  exit 1
fi

printf "\n# Executable located at: %s/release/file-ninja" "$FILENINJA_INSTALL"

printf "\n#"
printf "\n# ************* TEMPORARY INSTALL"
printf "\n#"
printf "\n#   To temporarily store file-ninja in your PATH run:"
printf "\n#"
printf "\n#       export PATH=%s/release:\$PATH" "$FILENINJA_INSTALL"

printf "\n#"
printf "\n# ************* PERMANENT INSTALL"
printf "\n#"
printf "\n#   To permanently add file-ninja to your PATH, add"
printf "\n#   the following to your ~/.bashrc or ~/.zshrc:"
printf "\n#"
printf "\n#       export PATH=%s/release:\$PATH" "$FILENINJA_INSTALL"
printf "\n#"
printf "\n#   Then restart your terminal or source ~/.bashrc or ~/.zshrc file."

printf "\n#"
printf "\n#"
printf "####################################################################################"
printf "\n"
printf "\n"
file-ninja --help
printf "\n"
printf "\n"
