use crate::*;

#[derive(Clone, Debug)]
pub struct Statistics {
    pub min_fitness: f32,
    pub max_fitness: f32,
    pub avg_fitness: f32,
    pub median_fitness: f32,
}

impl Statistics {
    pub fn new<I>(population: &[I]) -> Self
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let mut min_fitness = population[0].fitness();
        let mut max_fitness = min_fitness;
        let mut sum_fitness = 0.0;
        let mut fitnesses = Vec::with_capacity(population.len());

        for individual in population {
            let fitness = individual.fitness();

            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
            fitnesses.push(fitness);
        }

        fitnesses.sort_by(|a, b| a.total_cmp(b));
        let mid = fitnesses.len() / 2;
        let median_fitness = if fitnesses.len() % 2 == 0 {
            (fitnesses[mid - 1] + fitnesses[mid]) / 2.0
        } else {
            fitnesses[mid]
        };

        Self {
            min_fitness,
            max_fitness,
            avg_fitness: sum_fitness / (population.len() as f32),
            median_fitness,
        }
    }
}
