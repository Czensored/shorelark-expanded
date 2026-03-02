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
        Self::random_with_counts(rng, PREY_COUNT, PREDATOR_COUNT, FOOD_COUNT)
    }

    pub fn random_with_counts(
        rng: &mut dyn RngCore,
        prey_count: usize,
        predator_count: usize,
        food_count: usize,
    ) -> Self {
        let animals = (0..prey_count).map(|_| Animal::random(rng)).collect();
        let predators = (0..predator_count).map(|_| Predator::random(rng)).collect();
        let foods = (0..food_count).map(|_| Food::random(rng)).collect();

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
