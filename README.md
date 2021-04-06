
## This is my chess bot written in rust

Current features:

 * Finds allowed moves (Not everything implemented, check below)
 * Check for checks and mates
 * Check for en passant
 * Implement castling
 * Edwards notation support (partial)
 * Test cases based on edwards notation

Planned features:

 * Implement minmax of some sort
 * Alpha beta pruning
 * Other 

Dev build:
```
cargo run
```

Optimized build:
```
cargo build --release && time ./target/release/chess_bot
```
