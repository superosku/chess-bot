
## This is my chess bot written in rust

Current features:

 * Finds allowed moves (Not everything implemented, check below)

Planned features:

 * Check for checks and mates
 * Check for en passant
 * Implement castling
 * Implement minmax of some sort
 * Alpha beta pruning
 * Other 
 * Add chess board string serialization and deserialization support
 * Add automated testing

Dev build:
```
cargo run
```

Optimized build:
```
cargo build --release && time ./target/release/chess_bot
```
