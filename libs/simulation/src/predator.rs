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
        Self::random_with_config(rng, Eye::default().cells(), std::f32::consts::PI + std::f32::consts::FRAC_PI_4, 9, 1.0)
    }

    pub fn random_with_config(
        rng: &mut dyn RngCore,
        eye_cells: usize,
        fov_angle: f32,
        hidden_neurons: usize,
        speed_multiplier: f32,
    ) -> Self {
        let eye = Eye::with_cells_and_fov(eye_cells, fov_angle);
        let brain = Brain::random(rng, eye.cells(), hidden_neurons);

        Self::new(eye, brain, speed_multiplier, rng)
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    fn new(eye: Eye, brain: Brain, speed_multiplier: f32, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.r#gen(),
            rotation: rng.r#gen(),
            speed: 0.0011 * speed_multiplier,
            eye,
            brain,
            satiation: 0,
            color: PREDATOR_COLOR,
            alive: true,
        }
    }

    pub(crate) fn from_chromosome(
        chromosome: ga::Chromosome,
        eye_cells: usize,
        fov_angle: f32,
        hidden_neurons: usize,
        speed_multiplier: f32,
        rng: &mut dyn RngCore,
    ) -> Self {
        let eye = Eye::with_cells_and_fov(eye_cells, fov_angle);
        let brain = Brain::from_chromosome(chromosome, eye.cells(), hidden_neurons);

        Self::new(eye, brain, speed_multiplier, rng)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    pub fn color(&self) -> Rgba {
        self.color
    }
}
