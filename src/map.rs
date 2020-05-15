use super::*;

pub const SEA_LEVEL: f32 = 0.4;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Water,
    Ground,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub altitude: Vec<f32>,
    pub width: i32,
    pub height: i32,
    pub size: usize,
}

impl Map {
    pub fn idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn coord(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.width, idx as i32 / self.width)
    }

    pub fn new() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Ground; (WIDTH * HEIGHT) as usize],
            altitude: vec![0.; (WIDTH * HEIGHT) as usize],
            width: WIDTH,
            height: HEIGHT,
            size: (WIDTH * HEIGHT) as usize,
        };

        let mut noise = FastNoise::new();
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(10);
        noise.set_fractal_gain(0.4);
        noise.set_fractal_lacunarity(3.0);
        noise.set_frequency(0.02);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = map.idx(x, y);
                let n = noise.get_noise(x as f32, y as f32);
                let altitude = (n + 1.0) / 2.0;

                map.altitude[idx] = altitude;

                if altitude < SEA_LEVEL {
                    map.tiles[idx] = TileType::Water;
                }
            }
        }

        map
    }
}