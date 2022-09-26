# RUN

```
export CMC_APIKEY=<CMC_APIKEY>
export TELOXIDE_TOKEN=<TELOXIDE_TOKEN>
```

## telegram bot
`RUST_LOG=info cointoken_bot`

## command line
`cointoken-cli --from ETH --to USD --amount 1`

# ONLINE BOT
online bot: https://t.me/coin_token_bot

# USE cross build

```
cargo install cross
cross build --release --target x86_64-unknown-linux-musl
cross build --release --bin cointoken-cli --target x86_64-unknown-linux-musl
```

# Trouble

1. `failed to run custom build command for openssl-sys v0.9.39`

Cargo.toml
```
[dependencies]
openssl = { version = "0.10", features = ["vendored"] }
```