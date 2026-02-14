# Shorelark Genetic Algorithm

An interactive evolution sandbox: a genetic algorithm + feed-forward neural network controlling “birds” that learn to find food under configurable selection pressures. Built in Rust (WASM) with a TypeScript front-end for real-time visualization and experimentation.

## Live Demo

Try it here: [Shorelark Predator-Prey Simulation](https://isaacsalzman.com/shorelark-expanded/)

![Shorelark demo](./docs/shorelark-demo.gif)

## Problem Statement

While public genetic algorithm projects already exist and are helpful for demonstrating natural selection, most public algorithms rely on libraries that limit transparency and interactivity. This project aims to build on an existing evolution simulation and modifies and adds to the selection pressures in a variety of different ways to visualize the impact on the population. The goal of this project is to create a clear and modifiable sandbox where users can observe the impact of a variety of different selection pressures on the population.

This project also aims to mathematically quantify the improvement rate of the population over time given the input conditions to the simulation and the specific selection pressures being tested.

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

## Acknoledgements

This project was initially inspired by and prototyped from Patryk Wychowaniec’s “Learning to Fly” article/series:
- Guide: <https://pwy.io/posts/learning-to-fly-pt1/>
