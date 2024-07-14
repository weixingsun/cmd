# export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
# export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo build --release
target/release/cmd --mode=set_all_32c
