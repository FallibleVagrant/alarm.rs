Uses `cargo`, so simply:
```
cargo build --release
```
to get a binary.

alarm.rs will sleep to avoid spin-waiting the CPU for the system time constantly. It uses a sliding window to determine how long it should spend sleeping.
It is hardcoded never to be more than about a minute late.
