use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,

    #[wasm_bindgen(getter_with_clone)]
    pub predators: Vec<Predator>,

    #[wasm_bindgen(getter_with_clone)]
    pub foods: Vec<Food>,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub color: u32,
    pub alive: bool,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Predator {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub color: u32,
    pub alive: bool,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct GenerationStats {
    pub generation: u32,
    pub prey_min_fitness: f32,
    pub prey_max_fitness: f32,
    pub prey_avg_fitness: f32,
    pub prey_median_fitness: f32,
    pub prey_dead: u32,
    pub predator_min_fitness: f32,
    pub predator_max_fitness: f32,
    pub predator_avg_fitness: f32,
    pub predator_median_fitness: f32,
    pub predator_dead: u32,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }

    pub fn step(&mut self) -> Option<GenerationStats> {
        self.sim.step(&mut self.rng).map(GenerationStats::from)
    }

    pub fn fast_forward(&mut self) -> GenerationStats {
        GenerationStats::from(self.sim.fast_forward(&mut self.rng))
    }

    pub fn current_stats(&self) -> GenerationStats {
        GenerationStats::from(self.sim.current_statistics())
    }

    pub fn reset(
        &mut self,
        prey: u32,
        pred: u32,
        foods: u32,
        prey_n: u32,
        pred_n: u32,
        prey_p: u32,
        pred_p: u32,
        prey_fov: f32,
        pred_fov: f32,
        prey_speed_mul: f32,
        pred_speed_mul: f32,
    ) -> GenerationStats {
        let cfg = sim::SimulationConfig {
            prey_count: prey as usize,
            predator_count: pred as usize,
            food_count: foods as usize,
            prey_hidden_neurons: prey_n as usize,
            predator_hidden_neurons: pred_n as usize,
            prey_photoreceptors: prey_p as usize,
            predator_photoreceptors: pred_p as usize,
            prey_fov_angle: prey_fov,
            predator_fov_angle: pred_fov,
            prey_speed_multiplier: prey_speed_mul,
            predator_speed_multiplier: pred_speed_mul,
        };
        self.sim.reset_with_config(&mut self.rng, cfg);
        GenerationStats::from(self.sim.current_statistics())
    }
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();
        let predators = world.predators().iter().map(Predator::from).collect();
        let foods = world.foods().iter().map(Food::from).collect();

        Self {
            animals,
            predators,
            foods,
        }
    }
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        let c = animal.color();
        let color = u32::from_be_bytes([c.r, c.g, c.b, c.a]);

        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
            color,
            alive: animal.alive,
        }
    }
}

impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}

impl From<&sim::Predator> for Predator {
    fn from(predator: &sim::Predator) -> Self {
        let c = predator.color();
        let color = u32::from_be_bytes([c.r, c.g, c.b, c.a]);

        Self {
            x: predator.position().x,
            y: predator.position().y,
            rotation: predator.rotation().angle(),
            color,
            alive: predator.alive,
        }
    }
}

impl From<sim::Statistics> for GenerationStats {
    fn from(stats: sim::Statistics) -> Self {
        Self {
            generation: stats.generation as u32,
            prey_min_fitness: stats.prey_ga.min_fitness,
            prey_max_fitness: stats.prey_ga.max_fitness,
            prey_avg_fitness: stats.prey_ga.avg_fitness,
            prey_median_fitness: stats.prey_ga.median_fitness,
            prey_dead: stats.num_dead_prey,
            predator_min_fitness: stats.predator_ga.min_fitness,
            predator_max_fitness: stats.predator_ga.max_fitness,
            predator_avg_fitness: stats.predator_ga.avg_fitness,
            predator_median_fitness: stats.predator_ga.median_fitness,
            predator_dead: stats.num_dead_predators,
        }
    }
}
