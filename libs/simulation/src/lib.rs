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
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

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
const DEFAULT_PREY_NEURONS: usize = 9;
const DEFAULT_PREDATOR_NEURONS: usize = 9;
const DEFAULT_PREY_PHOTORECEPTORS: usize = 9;
const DEFAULT_PREDATOR_PHOTORECEPTORS: usize = 9;
const DEFAULT_SPEED_MULTIPLIER: f32 = 1.0;
const DEFAULT_FOV_ANGLE: f32 = PI + FRAC_PI_4;

#[derive(Clone, Debug)]
pub struct SimulationConfig {
    pub prey_count: usize,
    pub predator_count: usize,
    pub food_count: usize,
    pub prey_hidden_neurons: usize,
    pub predator_hidden_neurons: usize,
    pub prey_photoreceptors: usize,
    pub predator_photoreceptors: usize,
    pub prey_fov_angle: f32,
    pub predator_fov_angle: f32,
    pub prey_speed_multiplier: f32,
    pub predator_speed_multiplier: f32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            prey_count: PREY_COUNT,
            predator_count: PREDATOR_COUNT,
            food_count: FOOD_COUNT,
            prey_hidden_neurons: DEFAULT_PREY_NEURONS,
            predator_hidden_neurons: DEFAULT_PREDATOR_NEURONS,
            prey_photoreceptors: DEFAULT_PREY_PHOTORECEPTORS,
            predator_photoreceptors: DEFAULT_PREDATOR_PHOTORECEPTORS,
            prey_fov_angle: DEFAULT_FOV_ANGLE,
            predator_fov_angle: DEFAULT_FOV_ANGLE,
            prey_speed_multiplier: DEFAULT_SPEED_MULTIPLIER,
            predator_speed_multiplier: DEFAULT_SPEED_MULTIPLIER,
        }
    }
}

impl SimulationConfig {
    fn normalized(mut self) -> Self {
        self.prey_count = self.prey_count.max(1);
        self.predator_count = self.predator_count.max(1);
        self.food_count = self.food_count.max(1);
        self.prey_hidden_neurons = self.prey_hidden_neurons.max(1);
        self.predator_hidden_neurons = self.predator_hidden_neurons.max(1);
        self.prey_photoreceptors = self.prey_photoreceptors.max(1);
        self.predator_photoreceptors = self.predator_photoreceptors.max(1);
        self.prey_fov_angle = self.prey_fov_angle.clamp(0.01, TAU);
        self.predator_fov_angle = self.predator_fov_angle.clamp(0.01, TAU);
        self.prey_speed_multiplier = self.prey_speed_multiplier.max(0.01);
        self.predator_speed_multiplier = self.predator_speed_multiplier.max(0.01);
        self
    }
}

pub struct Simulation {
    world: World,
    config: SimulationConfig,
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
        Self::random_with_config(rng, SimulationConfig::default())
    }

    pub fn random_with_config(rng: &mut dyn RngCore, config: SimulationConfig) -> Self {
        let config = config.normalized();
        let world = Self::random_world_with_config(rng, &config);

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
            config,
            prey_ga,
            predator_ga,
            age: 0,
            generation: 0,
        }
    }

    fn random_world_with_config(rng: &mut dyn RngCore, cfg: &SimulationConfig) -> World {
        let animals = (0..cfg.prey_count)
            .map(|_| {
                Animal::random_with_config(
                    rng,
                    cfg.prey_photoreceptors,
                    cfg.prey_fov_angle,
                    cfg.prey_hidden_neurons,
                    cfg.prey_speed_multiplier,
                )
            })
            .collect();
        let predators = (0..cfg.predator_count)
            .map(|_| {
                Predator::random_with_config(
                    rng,
                    cfg.predator_photoreceptors,
                    cfg.predator_fov_angle,
                    cfg.predator_hidden_neurons,
                    cfg.predator_speed_multiplier,
                )
            })
            .collect();
        let foods = (0..cfg.food_count).map(|_| Food::random(rng)).collect();

        World {
            animals,
            predators,
            foods,
        }
    }

    pub fn reset_with_config(&mut self, rng: &mut dyn RngCore, config: SimulationConfig) {
        self.config = config.normalized();
        self.world = Self::random_world_with_config(rng, &self.config);
        self.age = 0;
        self.generation = 0;
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

    pub fn current_statistics(&self) -> Statistics {
        let prey_stats = current_fitness_stats(
            self.world
                .animals
                .iter()
                .filter(|animal| animal.alive)
                .map(|animal| 1.0 + animal.satiation as f32),
        );
        let predator_stats = current_fitness_stats(
            self.world
                .predators
                .iter()
                .filter(|predator| predator.alive)
                .map(|predator| 1.0 + predator.satiation as f32),
        );

        let num_dead_prey = self
            .world
            .animals
            .iter()
            .filter(|animal| !animal.alive)
            .count() as u32;
        let num_dead_predators = self
            .world
            .predators
            .iter()
            .filter(|predator| !predator.alive)
            .count() as u32;

        Statistics {
            generation: self.generation,
            prey_ga: prey_stats,
            predator_ga: predator_stats,
            num_dead_prey,
            num_dead_predators,
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

            let prey_speed_min = PREY_SPEED_MIN * self.config.prey_speed_multiplier;
            let prey_speed_max = PREY_SPEED_MAX * self.config.prey_speed_multiplier;
            animal.speed = (animal.speed + speed).clamp(prey_speed_min, prey_speed_max);
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

            let predator_speed_min = PREDATOR_SPEED_MIN * self.config.predator_speed_multiplier;
            let predator_speed_max = PREDATOR_SPEED_MAX * self.config.predator_speed_multiplier;
            predator.speed = (predator.speed + speed).clamp(predator_speed_min, predator_speed_max);
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
            .count() as u32;

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
            (0..self.config.prey_count)
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
                    median_fitness: 0.0,
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
                    median_fitness: 0.0,
                },
            )
        } else {
            self.predator_ga.evolve(rng, &predator_population)
        };

        self.world.animals = evolved_prey
            .into_iter()
            .map(|individual| {
                individual.into_animal(
                    self.config.prey_photoreceptors,
                    self.config.prey_fov_angle,
                    self.config.prey_hidden_neurons,
                    self.config.prey_speed_multiplier,
                    rng,
                )
            })
            .collect();
        self.world.predators = evolved_predators
            .into_iter()
            .map(|individual| {
                individual.into_predator(
                    self.config.predator_photoreceptors,
                    self.config.predator_fov_angle,
                    self.config.predator_hidden_neurons,
                    self.config.predator_speed_multiplier,
                    rng,
                )
            })
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

fn current_fitness_stats(values: impl Iterator<Item = f32>) -> ga::Statistics {
    let mut fitnesses = Vec::new();
    let mut sum = 0.0f32;
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    for value in values {
        sum += value;
        min = min.min(value);
        max = max.max(value);
        fitnesses.push(value);
    }

    if fitnesses.is_empty() {
        ga::Statistics {
            min_fitness: 0.0,
            max_fitness: 0.0,
            avg_fitness: 0.0,
            median_fitness: 0.0,
        }
    } else {
        fitnesses.sort_by(|a, b| a.total_cmp(b));
        let mid = fitnesses.len() / 2;
        let median_fitness = if fitnesses.len() % 2 == 0 {
            (fitnesses[mid - 1] + fitnesses[mid]) / 2.0
        } else {
            fitnesses[mid]
        };

        ga::Statistics {
            min_fitness: min,
            max_fitness: max,
            avg_fitness: sum / (fitnesses.len() as f32),
            median_fitness,
        }
    }
}
