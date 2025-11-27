use crate::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Statistics {
    pub generation: usize,
    pub ga: ga::Statistics,
    pub num_dead: u32,
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "generation {}, min={:.2}, max={:.2}, avg={:.2}, num_dead={}",
            self.generation,
            self.ga.min_fitness,
            self.ga.max_fitness,
            self.ga.avg_fitness,
            self.num_dead,
        )
    }
}
