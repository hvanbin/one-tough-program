# Jigsaw Puzzle Solver

A Rust program that solves "One Tough Puzzle" by Paul Lamond Games. The program uses a backtracking algorithm to find a valid arrangement of 9 puzzle pieces in a 3x3 grid.

## How it Works

The puzzle consists of 9 pieces, each with 4 edges that can be:
- **Tabs**: SpadeTab, DiamondTab, HeartTab, ClubTab
- **Slots**: SpadeSlot, DiamondSlot, HeartSlot, ClubSlot

Pieces can be rotated (0-3 positions) and must fit together such that:
- Tabs fit into corresponding slots (e.g., SpadeTab fits SpadeSlot)
- All 9 pieces are used exactly once
- The result forms a complete 3x3 grid

## Usage

### Run the solver:
```bash
cargo run
```

### Run the tests:
```bash
cargo test
```

### Run with verbose test output:
```bash
cargo test -- --nocapture
```

## Output Format

The solution is displayed as a 3x3 grid where each piece shows:
`EdgeTop,EdgeRight,EdgeBottom,EdgeLeft(Rotation)`

For example: `ST,DT,HS,DS(0)` means:
- Top edge: SpadeTab (ST)
- Right edge: DiamondTab (DT) 
- Bottom edge: HeartSlot (HS)
- Left edge: DiamondSlot (DS)
- Rotation: 0 (no rotation from original orientation)

The test suite includes:
- **Joint fitting logic**: Verifies tabs fit into correct slots
- **Piece creation and rotation**: Tests piece initialization and rotation math
- **Constraint checking**: Validates piece placement rules
- **Complete solver verification**: Ensures the solver finds valid solutions
- **Edge case handling**: Tests boundary conditions and error cases

## Building

Requires Rust 1.70+ (2021 edition)

```bash
cargo build --release
```

## Algorithm

The solver uses a recursive backtracking algorithm:
1. For each position in the 3x3 grid
2. Try each unused piece in each possible rotation
3. Check if the piece fits with already-placed neighbors
4. If it fits, recursively solve the next position
5. If no piece fits, backtrack and try the next option
6. Continue until a complete solution is found

The constraint checking ensures that adjacent pieces have compatible edges (tabs fit into slots of the same suit).
