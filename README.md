# control-box

Core Rust library for modeling and simulating control system signals.

## Features

- Signal trait for time-domain signal generation
- Built-in signal types: step, impulse, noise, superposition
- PT1 (first-order lag) element implementation
- Hysteresis modeling
- Modular design for easy extension

## Usage

Add to your workspace and use the signal types:

```rust
use cb_simulation_util::signal::{StepSignal, Signal};

let step = StepSignal::new(1.0, 0.0);
let value = step.value_at(0.5);
```

## Project Structure

- `src/signal/` — Signal traits signals implementations
- `src/plant/` — Element/plant traits and implementations
- `src/hysteresis.rs` — Hysteresis model

## Development

- Add new signal types in `src/signal/`
- Add new element types in `src/plant/`
- ci only checks for code sanity (build, test, doc/doc-test)

## License

MIT — see [`LICENSE.md`](LICENSE.md)