use crate::*;

pub struct PredatorIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl PredatorIndividual {
    pub fn from_predator(predator: &Predator) -> Self {
        Self {
            fitness: 1.0 + predator.satiation as f32,
            chromosome: predator.as_chromosome(),
        }
    }

    pub fn into_predator(
        self,
        eye_cells: usize,
        fov_angle: f32,
        hidden_neurons: usize,
        speed_multiplier: f32,
        rng: &mut dyn RngCore,
    ) -> Predator {
        Predator::from_chromosome(
            self.chromosome,
            eye_cells,
            fov_angle,
            hidden_neurons,
            speed_multiplier,
            rng,
        )
    }
}

impl ga::Individual for PredatorIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}
