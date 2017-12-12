## Board Game AI - Monte Carlo Tree Search

Rust implementation of Monte Carlo Tree Search for board game AIs.

Based on the existing implementation from [jbradberry/mcts](https://github.com/jbradberry/mcts).

### Testing

Pick the game to test by choosing which state to initialize:

```
// let mut state = NimState::new(10);
let mut state = AgricolaState::new(2);
```

Execute the game:

```
cd example-games/play-game
cargo run
```

### Games implemented 

(Nim)[./example-games/nim]
(Agricola - Still in testing)[./example-games/agricola]
