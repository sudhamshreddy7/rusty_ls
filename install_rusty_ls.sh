#!/bin/bash

# Update package list and install build essentials
sudo apt update
sudo apt install -y build-essential

# Install Rust
curl https://sh.rustup.rs -sSf | sh 
source "$HOME/.cargo/env"

# Verify Rust installation
rustc -V

# Clone the repository
# git clone https://github.com/sudhamshreddy7/rusty_ls.git

# Navigate to the project directory and compile the code
cd src
rustc main.rs -o rusty_ls

# Move the compiled binary to /usr/local/bin and set executable permissions
sudo mv rusty_ls /usr/local/bin/
sudo chmod +x /usr/local/bin/rusty_ls

# Done
echo "rusty_ls has been successfully installed."
