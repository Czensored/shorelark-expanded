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
    pub prey_dead: u32,
    pub predator_min_fitness: f32,
    pub predator_max_fitness: f32,
    pub predator_avg_fitness: f32,
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
            prey_dead: stats.num_dead_prey,
            predator_min_fitness: stats.predator_ga.min_fitness,
            predator_max_fitness: stats.predator_ga.max_fitness,
            predator_avg_fitness: stats.predator_ga.avg_fitness,
            predator_dead: stats.num_dead_predators,
        }
    }
}
