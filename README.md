# smith-tools 🔨

A collection of scripts and utilities to streamline development workflows, automate tasks, and boost productivity.

## Useful Commands and Tools

### Oh My Zsh

* [`ohmyzsh`](https://ohmyz.sh)

```shell
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
```

### SSH for GitHub

```shell
ssh-keygen -t ed25519
cat ~/.ssh/id_ed25519.pub

# Add New SSH Key
# https://github.com/settings/keys
ssh -T git@github.com
```

### Git Change Remotes

```shell
mdkir ~/projects
git clone git@github.com:raymondkfcheung/polkadot-sdk.git

cd ~/projects/polkadot-sdk
git remote set-url origin git@github.com:paritytech/polkadot-sdk.git

git remote add raymondkfcheung git@github.com:raymondkfcheung/polkadot-sdk.git

git remote -v
```

### Rust Installation

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "${HOME}/.cargo/env"
rustc -V
cargo -V
rustup -V

rustup install nightly
```

### Essential Tools for Building Polkadot SDK

```shell
cd ~/projects/polkadot-sdk

rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
rustup target add wasm32-unknown-unknown --toolchain stable-x86_64-unknown-linux-gnu
rustup target add wasm32v1-none --toolchain stable-x86_64-unknown-linux-gnu

sudo apt update
sudo apt list --upgradable
sudo apt upgrade

sudo apt install build-essential
sudo apt install protobuf-compiler
sudo apt install libc6-dev

# Optional - if that doesn't fully resolve the issue
sudo apt install clang libclang-dev llvm-dev libssl-dev

# Optional - if that doesn't fully resolve the issue
sudo apt install libgflags-dev libsnappy-dev zlib1g-dev libbz2-dev liblz4-dev libzstd-dev

cargo build
```

### Rsync

```shell
rsync -avz \
    --exclude '.git/' \
    --exclude '.idea/' \
    --exclude 'target/' \
    ~/projects/polkadot-sdk/ \
    ${SSH_USERNAME}@${SSH_HOST}:${REMOTE_PATH}/projects/polkadot-sdk/
```

### Rust Script

* [`rust-script`](https://github.com/fornwall/rust-script)

```shell
cargo install rust-script

cd ~/projects
git clone git@github.com:raymondkfcheung/smith-tools.git

alias run-tests="~/projects/smith-tools/src/rust/run_tests.rs"
```

### Cargo Commands

```shell
# Find version conflicts
cargo tree -i sp-io
cargo tree | grep "sp-io v" | sed 's/.*sp-io v\([0-9.]*\).*/v\1/' | sort | uniq -c
```
