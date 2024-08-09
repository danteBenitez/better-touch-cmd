# better-touch-cmd

A simple script to replace touch. Create files or dir depending on final `/`.

# Requirements

- [Rustc](https://www.rust-lang.org/learn/get-started)

# Releases

- [v0.1.0]()

# Setting up

```bash
git clone https://github.com/danteBenitez/better-touch-cmd
cd better-touch-cmd
cargo install --path .
```

# Recommended

- Move binary to `/usr/local/bin` to use it as a global command.

```bash
cargo build --release
sudo mv target/release/better-touch-cmd /usr/local/bin
```


