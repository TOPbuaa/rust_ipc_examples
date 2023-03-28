# Usage
ipmpsc example
```bash
cargo run --release --example ipmpsc_server /dev/shm/test
cargo run --release --example ipmpsc_client /dev/shm/test
```

Unix Domain Socket example
```bash
cargo run --release --example uds_server /tmp/test_domain.socket
cargo run --release --example uds_client /tmp/test_domain.socket
```