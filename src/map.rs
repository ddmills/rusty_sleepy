use super::*;

pub const SEA_LEVEL: f32 = 0.5;

#[derive(PartialEq, Copy, Clone)]
pub enum Biome {
    Water,
    IceSheet,
    ColdDesert,
    Tundra,
    BorealForest,
    Grassland,
    Woodland,
    TemperateForest,
    Rainforest,
    Desert,
    Savanna,
}

pub struct Map {
    pub tiles: Vec<Biome>,
    pub elevation: Vec<f32>,
    pub precipitation: Vec<f32>,
    pub temperature: Vec<f32>,
    pub biome: Vec<Biome>,
    pub hsv: Vec<HSV>,
    pub width: i32,
    pub height: i32,
    pub size: usize,
}

fn hsv(e: f32) -> HSV {
    let v =  e / 1.5 + 0.25;
    if e < 0.6 { return HSV::from_f32(0.601, 0.5, v); }
    if e < 0.65 { return HSV::from_f32(0.136, 0.5, 0.8); }
    if e < 0.8 { return HSV::from_f32(0.333, 0.5, v); }
    if e < 0.85 { return HSV::from_f32(0.393, 0.5, v); }
    if e < 0.9 { return HSV::from_f32(0.206, 0.5, v); }
    if e < 0.95 { return HSV::from_f32(0.065, 0.5, v); }
    HSV::from_f32(0.0, 0.0, v)
}

fn temp(v: f32) -> f32 {
    (v * std::f32::consts::PI).sin()
}

impl Map {
    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn coord(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.width, idx as i32 / self.width)
    }

    pub fn biome(e: f32, m: f32, t: f32) -> Biome {
        if e < 0.4 {
            if t < 0.5 {
                return Biome::Water;
            }
            return Biome::IceSheet;
        }

        if t < 0.15 {
            return Biome::ColdDesert;
        }

        if t < 0.25 {
            return Biome::Tundra;
        }

        if t < 0.75 {
            if t < 0.5 {
                if m < 0.5 {
                    return Biome::BorealForest;
                }
            }
            if m < 0.1 {
                return Biome::Grassland;
            }
            if m < 0.25 {
                return Biome::Woodland;
            }

            return Biome::TemperateForest;
        }

        if m < 0.25 {
            return Biome::Desert;
        }

        if m < 0.75 {
            return Biome::Savanna;
        }

        return Biome::Rainforest;
    }

    pub fn generate(&mut self, seed: u64) {
        let mut noise_elevation = FastNoise::new();
        noise_elevation.set_seed(seed);
        noise_elevation.set_noise_type(NoiseType::SimplexFractal);
        noise_elevation.set_fractal_type(FractalType::FBM);
        // noise_elevation.set_interp(Interp::Quintic);
        noise_elevation.set_fractal_octaves(4);
        noise_elevation.set_fractal_gain(0.4);
        noise_elevation.set_fractal_lacunarity(2.5);
        noise_elevation.set_frequency(0.03);

        let mut noise_precipitation = FastNoise::new();
        noise_precipitation.set_seed(seed);
        noise_precipitation.set_noise_type(NoiseType::SimplexFractal);
        noise_precipitation.set_fractal_type(FractalType::Billow);
        // noise_precipitation.set_interp(Interp::Quintic);
        noise_precipitation.set_fractal_octaves(5);
        noise_precipitation.set_fractal_gain(0.3);
        noise_precipitation.set_fractal_lacunarity(1.0);
        noise_precipitation.set_frequency(0.008);

        let mut min_elevation = 0.0;
        let mut max_elevation = 0.0;
        let mut min_precipitation = 0.0;
        let mut max_precipitation = 0.0;
        let mut nz_elevation = vec![0.; (WIDTH * HEIGHT) as usize];
        let mut nz_precipitation = vec![0.; (WIDTH * HEIGHT) as usize];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let e = noise_elevation.get_noise(x as f32, y as f32);
                let m = noise_precipitation.get_noise(x as f32, y as f32);

                if e < min_elevation {
                    min_elevation = e;
                }
                if e > max_elevation {
                    max_elevation = e;
                }
                if m < min_precipitation {
                    min_precipitation = m;
                }
                if m > max_precipitation {
                    max_precipitation = m;
                }

                nz_elevation[self.idx(x, y)] = e;
                nz_precipitation[self.idx(x, y)] = m;
            }
        }

        let range_elevation = max_elevation - min_elevation;
        let range_precipitation = max_precipitation - min_precipitation;

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = self.idx(x, y);
                let nz_e = nz_elevation[idx];
                let nz_m = nz_precipitation[idx];
                let elevation = (nz_e - min_elevation) / range_elevation;
                let precipitation = (nz_m - min_precipitation) / range_precipitation;
                let temperature = temp((y as f32) / (HEIGHT as f32));

                self.elevation[idx] = elevation.powf(1.2);
                // self.elevation[idx] = elevation;
                self.precipitation[idx] = precipitation;
                self.temperature[idx] = (temperature + (1.0 - elevation.powf(2.0))) / 2.0;
                // self.temperature[idx] = temperature;
                self.hsv[idx] = hsv(elevation);
            }
        }
    }

    pub fn new() -> Map {
        Map {
            tiles: vec![Biome::Water; (WIDTH * HEIGHT) as usize],
            elevation: vec![0.; (WIDTH * HEIGHT) as usize],
            precipitation: vec![0.; (WIDTH * HEIGHT) as usize],
            temperature: vec![0.; (WIDTH * HEIGHT) as usize],
            biome: vec![Biome::Water; (WIDTH * HEIGHT) as usize],
            hsv: vec![HSV::from_f32(0.0, 1.0, 1.0); (WIDTH * HEIGHT) as usize],
            width: WIDTH,
            height: HEIGHT,
            size: (WIDTH * HEIGHT) as usize,
        }
    }
}
