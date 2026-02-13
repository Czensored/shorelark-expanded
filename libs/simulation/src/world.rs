use crate::*;

pub const PREY_COUNT: usize = 40;
pub const PREDATOR_COUNT: usize = 6;
pub const FOOD_COUNT: usize = 60;

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) predators: Vec<Predator>,
    pub(crate) foods: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..PREY_COUNT).map(|_| Animal::random(rng)).collect();
        let predators = (0..PREDATOR_COUNT).map(|_| Predator::random(rng)).collect();

        let foods = (0..FOOD_COUNT).map(|_| Food::random(rng)).collect();

        // ^ Our algorithm allows for animals and foods to overlap, so
        // | it's hardly ideal - but good enough for our purposes.
        // |
        // | A more complex solution could be based off of e.g.
        // | Poisson disk sampling:
        // |
        // | https://en.wikipedia.org/wiki/Supersampling
        // ---

        Self {
            animals,
            predators,
            foods,
        }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn predators(&self) -> &[Predator] {
        &self.predators
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}
