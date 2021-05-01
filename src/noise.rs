use noise::{Fbm, NoiseFn, MultiFractal, Seedable};
use noise::utils::{PlaneMapBuilder, NoiseMapBuilder};

pub struct Noise {
    pub noise_map: Vec<f64>,
}

impl Noise {
    pub fn new(resolution: usize, octaves: usize, frequency: f64, lacunarity: f64, seed_value: u32) -> Noise {
        let mut noise_map = vec![];
        let fbm = Fbm::new()
            .set_seed(seed_value)
            .set_octaves(octaves)
            .set_frequency(frequency)
            .set_lacunarity(lacunarity);
        PlaneMapBuilder::new(&fbm)
            .set_size(resolution, resolution)
            .build();
        let mut highest = f64::MIN;
        let mut lowest = f64::MAX;
        for i in 0..resolution {
            for j in 0..resolution {
                let hightval = fbm.get([j as f64, i as f64]);
                noise_map.push(hightval);
                if hightval > highest {
                    highest = hightval;
                }
                if hightval < lowest {
                    lowest = hightval;
                }
            }
        }
        lowest = -lowest;
        highest = highest + lowest;

        for i in 0..noise_map.len() {
            noise_map[i] = (((noise_map[i] + lowest) / highest) * 2.0) - 1.0;
            //eprintln!("{}", noise_map[i]);
        }

        Noise {
            noise_map: noise_map,
        }
    }
}