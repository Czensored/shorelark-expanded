# Shorelark Genetic Algorithm

Followed a [guide](https://pwy.io/posts/learning-to-fly-pt1/) for this implementation.

## How to Build

```bash
# Build Rust code
cd libs/simulation-wasm
wasm-pack build --target bundler --release

# Build TypeScript code
cd ../../www
npm install

# Start the application
npm run start
```

## Explanation

This project implements a genetic algorithm in combination with a [Feedforward neural network](https://en.wikipedia.org/wiki/Feedforward_neural_network) (FFNN). The network receives input from each bird’s visual sensors: the bird’s field of view is divided into discrete “eye cells,” and each cell provides a numeric value representing the distance to the nearest food source. These inputs are processed by the network to produce two outputs: the bird’s forward/backward acceleration and its rotational acceleration.
