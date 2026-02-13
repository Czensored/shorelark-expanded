use crate::*;

const PREDATOR_COLOR: Rgba = Rgba {
    r: 255,
    g: 90,
    b: 90,
    a: 255,
};

#[derive(Debug)]
pub struct Predator {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: Brain,
    pub(crate) satiation: usize,
    pub(crate) color: Rgba,
    pub alive: bool,
}

impl Predator {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, eye.cells());

        Self::new(eye, brain, rng)
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.r#gen(),
            rotation: rng.r#gen(),
            speed: 0.0011,
            eye,
            brain,
            satiation: 0,
            color: PREDATOR_COLOR,
            alive: true,
        }
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, eye.cells());

        Self::new(eye, brain, rng)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    pub fn color(&self) -> Rgba {
        self.color
    }
}
