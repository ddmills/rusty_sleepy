use super::*;

pub const SEA_LEVEL: f32 = 0.55;

pub struct World {
    pub tiles: Vec<f32>,
    pub elevation: Vec<f32>,
    pub precipitation: Vec<f32>,
    pub temperature: Vec<f32>,
    pub width: u32,
    pub height: u32,
    pub size: usize,
    pub seed: u32,
    nz_elevation: FastNoise,
    nz_precipitation: FastNoise,
}

fn normalize_noize(v: f32) -> f32 {
    (v + 1.0) / 2.0
}

impl World {
    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn tile(&self, idx: usize) -> (u32, u32) {
        (idx as u32 % self.width, idx as u32 / self.width)
    }

    pub fn compute_elevation(&self, x: f32, y: f32) -> f32 {
        let v = self.nz_elevation.get_noise(x, y);
        normalize_noize(v)
    }

    pub fn compute_precipitation(&self, x: f32, y: f32) -> f32 {
        let v = self.nz_precipitation.get_noise(x, y);
        (1.0 - normalize_noize(v)).powf(1.5)
    }

    pub fn compute_temperature(&self, x: f32, y: f32) -> f32 {
        let sun = (((y as f32) / ((HEIGHT - 1) as f32)) * std::f32::consts::PI).sin();
        let ele = self.compute_elevation(x, y);

        sun * (1.0 - ele.powf(3.0))
    }

    pub fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
        self.nz_elevation.set_seed(seed as u64);
        self.nz_elevation.set_noise_type(NoiseType::SimplexFractal);
        self.nz_elevation.set_fractal_type(FractalType::FBM);
        self.nz_elevation.set_interp(Interp::Quintic);
        self.nz_elevation.set_fractal_octaves(8);
        self.nz_elevation.set_fractal_gain(0.3);
        self.nz_elevation.set_fractal_lacunarity(3.0);
        self.nz_elevation.set_frequency(0.035);

        self.nz_precipitation.set_seed(seed as u64);
        self.nz_precipitation
            .set_noise_type(NoiseType::SimplexFractal);
        self.nz_precipitation.set_fractal_type(FractalType::FBM);
        self.nz_precipitation.set_fractal_octaves(6);
        self.nz_precipitation.set_fractal_gain(0.5);
        self.nz_precipitation.set_fractal_lacunarity(1.3);
        self.nz_precipitation.set_frequency(0.03);
        self.generate();
    }

    pub fn generate(&mut self) {
        let mut x = 0;
        let mut y = 0;

        for (i, _tile) in self.tiles.iter().enumerate() {
            let xf32 = x as f32;
            let yf32 = y as f32;

            self.elevation[i] = self.compute_elevation(xf32, yf32);
            self.precipitation[i] = self.compute_precipitation(xf32, yf32);
            self.temperature[i] = self.compute_temperature(xf32, yf32);

            x += 1;
            if x > WIDTH - 1 {
                x = 0;
                y += 1;
            }
        }
    }

    pub fn new() -> World {
        let mut world = World {
            tiles: vec![0.; (WIDTH * HEIGHT) as usize],
            elevation: vec![0.; (WIDTH * HEIGHT) as usize],
            precipitation: vec![0.; (WIDTH * HEIGHT) as usize],
            temperature: vec![0.; (WIDTH * HEIGHT) as usize],
            width: WIDTH,
            height: HEIGHT,
            size: (WIDTH * HEIGHT) as usize,
            nz_elevation: FastNoise::new(),
            nz_precipitation: FastNoise::new(),
            seed: 0,
        };

        world.set_seed(world.seed);

        world
    }
}
