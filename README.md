## Building for bare metal

to compile, you need to add the bare metal target to rustup

```zsh
rustup target add thumbv7em-none-eabihf
```

then, just build the project for the desired target

```zsh
cargo build --target thumbv7em-none-eabihf
```
