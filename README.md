# RSW

## Autoreloading

```bash
cargo install systemfd cargo-watch
```

```toml
[dependencies]
listenfd = "0.3.3"
```

```bash
# run server
systemfd --no-pid -s http::4000 -- cargo watch -x run
```
