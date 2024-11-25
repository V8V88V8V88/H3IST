# H3IST

A terminal elevator simulator in Rust, written to learn async Rust with tokio.

The default building has 4 elevators and 32 floors. Each elevator speeds up and slows down with a fixed acceleration, works out its stopping distance so it halts at the floor, and opens its doors when it arrives. The screen redraws every 100 ms with each car's floor, direction and door state.

When a request comes in, the scheduler picks the cheapest idle or passing elevator by distance and sends it to the caller's floor. You can add requests yourself or generate up to 10 random ones.

## Run

```bash
cargo run --release
```

Uses `cargo-features = ["edition2024"]`, so it needs a nightly toolchain or a Rust release with edition 2024 stable.

## Status

Unfinished.

- The menu loop throws away what you type, so options 1–3 don't respond yet.
- Elevators only go to the pickup floor. They don't carry passengers on to their destination.
- `OptimizedScan` in `algorithms.rs` sketches a SCAN-style scheduler that weighs energy against wait time, but its cost functions are still stubs.
