# Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
export PATH="$HOME/.cargo/bin:$PATH"
```

## Install Solana

```
sh -c "$(curl -sSfL https://release.solana.com/v1.9.4/install)"
export PATH="/home/gitpod/.local/share/solana/install/active_release/bin:$PATH"
```

localpubkey = 7THZBBxW5xN2sL6P4kQbL3LwKwgoJLYzK3kwb8o1csR5

## libudev install
```
sudo apt install libudev
```

## Anchor install

```
cargo install --git https://github.com/project-serum/anchor anchor-cli --locked
```

