use super::*;

pub const SEA_LEVEL: f32 = 0.55;

pub struct World {
    pub tiles: Vec<f32>,
    pub elevation: Vec<f32>,
    pub precipitation: Vec<f32>,
    pub temperature: Vec<f32>,
    pub width: i32,
    pub height: i32,
    pub size: usize,
    nz_elevation: FastNoise,
    nz_precipitation: FastNoise,
}

fn temp(v: f32) -> f32 {
    (v * std::f32::consts::PI).sin()
}

pub fn sphere_vertex(lat: f32, lon: f32) -> (f32, f32, f32) {
    (
        f32::cos(lat) * f32::cos(lon),
        f32::cos(lat) * f32::sin(lon),
        f32::sin(lat),
    )
}

fn normalize_noize(v: f32) -> f32 {
    (v + 1.0) / 2.0
}

impl World {
    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn tile_to_lat_lon(&self, x: i32, y: i32) -> (f32, f32) {
        let lon = (((x as f32 / self.width as f32) * 360.0) - 180.0) * 0.017_453_3;
        let lat = (((y as f32 / self.height as f32) * 180.0) - 90.0) * 0.017_453_3;

        (lat, lon)
    }

    pub fn tile_to_sphere(&self, x: i32, y: i32) -> (f32, f32, f32) {
        let (lat, lon) = self.tile_to_lat_lon(x, y);

        sphere_vertex(lat, lon)
    }

    pub fn tile(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.width, idx as i32 / self.width)
    }

    pub fn elevation(&self, x: i32, y: i32) -> f32 {
        let coord = self.tile_to_sphere(x, y);
        let v = self.nz_elevation.get_noise3d(coord.0, coord.1, coord.2);
        normalize_noize(v)
    }

    pub fn precipitation(&self, x: i32, y: i32) -> f32 {
        let coord = self.tile_to_sphere(x, y);
        let v = self.nz_precipitation.get_noise3d(coord.0, coord.1, coord.2);
        (1.0 - normalize_noize(v)).powf(1.5)
    }

    pub fn temperature(&self, x: i32, y: i32) -> f32 {
        let sun = temp((y as f32) / ((HEIGHT - 1) as f32));
        let ele = self.elevation(x, y);

        sun * (1.0 - ele.powf(3.0))
    }

    pub fn generate(&mut self, seed: u32) {
        self.nz_elevation.set_seed(seed as u64);
        self.nz_elevation.set_noise_type(NoiseType::SimplexFractal);
        self.nz_elevation.set_fractal_type(FractalType::FBM);
        self.nz_elevation.set_interp(Interp::Quintic);
        self.nz_elevation.set_fractal_octaves(8);
        self.nz_elevation.set_fractal_gain(0.3);
        self.nz_elevation.set_fractal_lacunarity(2.5);
        self.nz_elevation.set_frequency(0.6);

        self.nz_precipitation.set_seed(seed as u64);
        self.nz_precipitation.set_noise_type(NoiseType::SimplexFractal);
        self.nz_precipitation.set_fractal_type(FractalType::FBM);
        // self.nz_precipitation.set_interp(Interp::Quintic);
        self.nz_precipitation.set_fractal_octaves(3);
        self.nz_precipitation.set_fractal_gain(0.5);
        self.nz_precipitation.set_fractal_lacunarity(0.5);
        self.nz_precipitation.set_frequency(0.7);
    }

    pub fn new() -> World {
        World {
            tiles: vec![0.; (WIDTH * HEIGHT) as usize],
            elevation: vec![0.; (WIDTH * HEIGHT) as usize],
            precipitation: vec![0.; (WIDTH * HEIGHT) as usize],
            temperature: vec![0.; (WIDTH * HEIGHT) as usize],
            width: WIDTH,
            height: HEIGHT,
            size: (WIDTH * HEIGHT) as usize,
            nz_elevation: FastNoise::new(),
            nz_precipitation: FastNoise::new(),
        }
    }
}
