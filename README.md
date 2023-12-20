#

## Prepare the testnet

Start testnet in the terminal:
```
RUST_LOG=safenode,safe cargo run --bin testnet --features local-discovery -- --build-node --build-faucet --interval 100
```

In another terminal, put some money in your wallet from the faucet:
```
cargo run --release --bin safe --features local-discovery wallet get-faucet http://localhost:8000
```

Test uploading a file:
```
cargo run --release --bin safe --features local-discovery -- files upload '/home/tom/Music/01 - Pretty Green.MP3'
```

Create a register:
```
cargo run --release --bin safe --features local-discovery -- register create -n myregister
```

Get the register created above:
```
cargo run --release --bin safe --features local-discovery -- register get -n myregister
```

Edit the above register:
```
cargo run --release --bin safe --features local-discovery -- register edit -n myregister somename
```

Example register app instance one:
```
cargo run --release --example registers --features local-discovery -- --user alice --reg-nickname myregister
```

Example register app instance two:
```
cargo run --release --example registers --features local-discovery -- --user bob --reg-nickname myregister
```

