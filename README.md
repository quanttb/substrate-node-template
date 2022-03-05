# Substrate Node Template

## Execution

- Check that code compiles:
```sh
cargo check -p node-template-runtime
```

- Compile the node template:
```sh
cargo build --release
```

- Start the node in development mode:
```sh
./target/release/node-template --dev --ws-port 9944 --rpc-port 9933 --ws-external --rpc-external
```

## Testing

[Polkadot{.js} apps wallet](https://polkadot.js.org/apps/?rpc=wss://rpc.tauhu.cloud#/explorer)

## Contribution

Your contribution is welcome and greatly appreciated. Please contribute your fixes and new features via a pull request.
Pull requests and proposed changes will then go through a code review and once approved will be merged into the project.

If you like my work, please leave me a star :)
