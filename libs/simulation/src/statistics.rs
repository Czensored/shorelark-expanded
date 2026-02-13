use crate::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Statistics {
    pub generation: usize,
    pub prey_ga: ga::Statistics,
    pub predator_ga: ga::Statistics,
    pub num_dead_prey: u32,
    pub num_dead_predators: u32,
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "generation {}, prey(min={:.2}, max={:.2}, avg={:.2}, dead={}), predators(min={:.2}, max={:.2}, avg={:.2}, dead={})",
            self.generation,
            self.prey_ga.min_fitness,
            self.prey_ga.max_fitness,
            self.prey_ga.avg_fitness,
            self.num_dead_prey,
            self.predator_ga.min_fitness,
            self.predator_ga.max_fitness,
            self.predator_ga.avg_fitness,
            self.num_dead_predators,
        )
    }
}
