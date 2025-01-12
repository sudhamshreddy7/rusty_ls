#!/bin/bash

# Update Homebrew (macOS package manager)
brew update

# Install build essentials
xcode-select --install

# Install Rust using rustup
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"

# Verify Rust installation
rustc -V

# Clone the repository (optional, uncomment the next line if needed)
# git clone https://github.com/sudhamshreddy7/rusty_ls.git

# Navigate to the project directory and compile the code
cd src || exit
rustc main.rs -o rusty_ls

# Move the compiled binary to /usr/local/bin and set executable permissions
sudo mv rusty_ls /usr/local/bin/
sudo chmod +x /usr/local/bin/rusty_ls

# Done
echo "rusty_ls has been successfully installed."
