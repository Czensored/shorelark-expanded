use crate::*;

#[derive(Debug)]
pub struct Brain {
    pub(crate) nn: nn::Network,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, input_neurons: usize) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(input_neurons)),
        }
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, input_neurons: usize) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(input_neurons), chromosome),
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    pub fn color(&self) -> Rgba {
        let bytes = self.as_chromosome().to_bytes();
        rgba_from_bytes(&bytes)
    }

    fn topology(input_neurons: usize) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology { neurons: input_neurons },
            nn::LayerTopology {
                neurons: 2 * input_neurons,
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}
