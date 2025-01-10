# rusty_ls

### Prerequisites:
1. Ubuntu with sudo permissions
## Steps invloved in creating user-level-system-call:
1. Run the following commands:
```bash
sudo apt update
sudo apt install build-essential
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
rustc -V
git clone https://github.com/sudhamshreddy7/rusty_ls.git
cd rusty_ls
cd src
rustc main.rs -o rusty_ls
sudo mv rusty_ls /usr/local/bin/
sudo chmod +x /usr/local/bin/rusty_ls
```
3. To test the command go to any folder you and type below command:
```bash
rusty_ls
```