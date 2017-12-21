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
cargo run --release
```

### Adjusting difficulty

In `example-games/play-game/src/main.rs`, adjusting the `iterations` number will increase the number of games played by the AI before making a decision.

```
iterations = 10001;
best_action = UCT(arena, state.clone(), iterations);
```

The following is a table of iterations to time per selection:

```
1000 - 1 second
10000 - 20 seconds
100000 - 260 seconds
```

### Games implemented 

[Nim](./example-games/nim)

[Agricola - Still in testing](./example-games/agricola)
