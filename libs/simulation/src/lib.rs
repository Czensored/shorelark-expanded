mod animal;
mod animal_individual;
mod brain;
mod color;
mod eye;
mod food;
mod predator;
mod predator_individual;
mod statistics;
mod world;

pub use self::{
    animal::*, animal_individual::*, brain::*, color::*, eye::*, food::*, predator::*,
    predator_individual::*, statistics::*, world::*,
};

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};

// FRAC_PI_2 = PI / 2.0; a convenient shortcut
use std::f32::consts::FRAC_PI_2;

const PREY_SPEED_MIN: f32 = 0.0007;
const PREY_SPEED_MAX: f32 = 0.0035;
const PREY_SPEED_ACCEL: f32 = 0.2;
const PREY_ROTATION_ACCEL: f32 = FRAC_PI_2 / 3.0;

const PREDATOR_SPEED_MIN: f32 = 0.0006;
const PREDATOR_SPEED_MAX: f32 = 0.0032;
const PREDATOR_SPEED_ACCEL: f32 = 0.25;
const PREDATOR_ROTATION_ACCEL: f32 = FRAC_PI_2 / 2.5;
const PREY_PREDATOR_VISION_GAIN: f32 = 2.5;

const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    world: World,
    prey_ga: ga::GeneticAlgorithm<
        ga::RouletteWheelSelection,
        ga::UniformCrossover,
        ga::GaussianMutation,
    >,
    predator_ga: ga::GeneticAlgorithm<
        ga::RouletteWheelSelection,
        ga::UniformCrossover,
        ga::GaussianMutation,
    >,
    age: usize,
    generation: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);

        let prey_ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection,
            ga::UniformCrossover,
            ga::GaussianMutation::new(0.01, 0.3),
        );

        let predator_ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection,
            ga::UniformCrossover,
            ga::GaussianMutation::new(0.01, 0.3),
        );

        Self {
            world,
            prey_ga,
            predator_ga,
            age: 0,
            generation: 0,
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.process_prey_food_collisions(rng);
        self.process_predator_prey_collisions();
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    pub fn fast_forward(&mut self, rng: &mut dyn RngCore) -> Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }

    fn process_prey_food_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            if !animal.alive {
                continue;
            }
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.r#gen();
                }
            }
        }
    }

    fn process_predator_prey_collisions(&mut self) {
        for predator in &mut self.world.predators {
            if !predator.alive {
                continue;
            }

            for animal in &mut self.world.animals {
                if !animal.alive {
                    continue;
                }

                let distance = na::distance(&predator.position, &animal.position);
                if distance <= 0.012 {
                    animal.alive = false;
                    animal.speed = 0.0;
                    predator.satiation += 3;
                }
            }
        }
    }

    fn process_brains(&mut self) {
        let predator_positions: Vec<_> = self
            .world
            .predators
            .iter()
            .filter(|predator| predator.alive)
            .map(|predator| predator.position)
            .collect();

        for animal in &mut self.world.animals {
            if !animal.alive {
                continue;
            }
            let mut vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);
            let predator_vision = animal.eye.process_vision_positions(
                animal.position,
                animal.rotation,
                predator_positions.iter().copied(),
            );
            vision.extend(
                predator_vision
                    .into_iter()
                    .map(|cell| cell * PREY_PREDATOR_VISION_GAIN),
            );

            let response = animal.brain.nn.propagate(vision);

            let speed = response[0].clamp(-PREY_SPEED_ACCEL, PREY_SPEED_ACCEL);
            let rotation = response[1].clamp(-PREY_ROTATION_ACCEL, PREY_ROTATION_ACCEL);

            animal.speed = (animal.speed + speed).clamp(PREY_SPEED_MIN, PREY_SPEED_MAX);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
        }

        let prey_positions: Vec<_> = self
            .world
            .animals
            .iter()
            .filter(|animal| animal.alive)
            .map(|animal| animal.position)
            .collect();

        for predator in &mut self.world.predators {
            if !predator.alive {
                continue;
            }

            let vision = predator.eye.process_vision_positions(
                predator.position,
                predator.rotation,
                prey_positions.iter().copied(),
            );

            let response = predator.brain.nn.propagate(vision);
            let speed = response[0].clamp(-PREDATOR_SPEED_ACCEL, PREDATOR_SPEED_ACCEL);
            let rotation = response[1].clamp(-PREDATOR_ROTATION_ACCEL, PREDATOR_ROTATION_ACCEL);

            predator.speed = (predator.speed + speed).clamp(PREDATOR_SPEED_MIN, PREDATOR_SPEED_MAX);
            predator.rotation = na::Rotation2::new(predator.rotation.angle() + rotation);
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            if !animal.alive {
                continue;
            }
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }

        for predator in &mut self.world.predators {
            if !predator.alive {
                continue;
            }

            predator.position += predator.rotation * na::Vector2::new(0.0, predator.speed);
            predator.position.x = na::wrap(predator.position.x, 0.0, 1.0);
            predator.position.y = na::wrap(predator.position.y, 0.0, 1.0);
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> Statistics {
        self.age = 0;

        let num_dead_prey = self
            .world
            .animals
            .iter()
            .filter(|animal| !animal.alive)
            .count() as u32; // assuming there is <4 billion dead animals

        let num_dead_predators = self
            .world
            .predators
            .iter()
            .filter(|predator| !predator.alive)
            .count() as u32;

        let alive_prey: Vec<_> = self
            .world
            .animals
            .iter()
            .filter(|animal| animal.alive)
            .collect();
        let prey_population: Vec<_> = if alive_prey.is_empty() {
            Vec::new()
        } else {
            (0..PREY_COUNT)
                .map(|i| AnimalIndividual::from_animal(alive_prey[i % alive_prey.len()]))
                .collect()
        };

        let predator_population: Vec<_> = self
            .world
            .predators
            .iter()
            .filter(|predator| predator.alive)
            .map(PredatorIndividual::from_predator)
            .collect();

        let (evolved_prey, prey_stats) = if prey_population.is_empty() {
            (
                Vec::new(),
                ga::Statistics {
                    min_fitness: 0.0,
                    max_fitness: 0.0,
                    avg_fitness: 0.0,
                },
            )
        } else {
            self.prey_ga.evolve(rng, &prey_population)
        };
        let (evolved_predators, predator_stats) = if predator_population.is_empty() {
            (
                Vec::new(),
                ga::Statistics {
                    min_fitness: 0.0,
                    max_fitness: 0.0,
                    avg_fitness: 0.0,
                },
            )
        } else {
            self.predator_ga.evolve(rng, &predator_population)
        };

        self.world.animals = evolved_prey
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();
        self.world.predators = evolved_predators
            .into_iter()
            .map(|individual| individual.into_predator(rng))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.r#gen();
        }

        let generation = self.generation;
        self.generation += 1;

        Statistics {
            generation,
            prey_ga: prey_stats,
            predator_ga: predator_stats,
            num_dead_prey,
            num_dead_predators,
        }
    }
}
