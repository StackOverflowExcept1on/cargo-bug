### how to reproduce?

```bash
cargo install cargo-http-registry
# in the background
rm -rf /tmp/my-registry && cargo-http-registry --addr 127.0.0.1:35503 /tmp/my-registry &
cargo publish -p demo-a && cargo publish -p demo-b
fg
Ctrl+C
```
