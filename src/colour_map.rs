use crate::vec3::Vec3;

pub struct ColourMap {
    colour_vec: Vec<ColourData>,
    default_colour: Vec3,
}

pub struct ColourData {
    cutoff: f64,
    colour: Vec3,
}

impl ColourMap {
    #[allow(dead_code)]
    pub fn new(colour_data: Vec<ColourData>, default_colour_: Vec3) -> Self {
        ColourMap {
            colour_vec: colour_data,
            default_colour: default_colour_,
        }
    }

    pub fn new_default() -> Self {
        let mut colours = vec![];
        colours.push(ColourData {cutoff: 0.75, colour: Vec3::new(1.0, 1.0, 1.0)});
        colours.push(ColourData {cutoff: 0.2, colour: Vec3::new(0.4, 0.5, 0.5)});
        colours.push(ColourData {cutoff: -0.25, colour: Vec3::new(0.2, 0.9, 0.4)});
        colours.push(ColourData {cutoff: -0.55, colour: Vec3::new(0.1, 0.6, 0.2)});
        colours.push(ColourData {cutoff: -0.7, colour: Vec3::new(0.05, 0.3, 0.85)});

        ColourMap {
            colour_vec: colours,
            default_colour: Vec3::new(0.05, 0.15, 0.6),
        }
    }

    pub fn to_colour(&self, value: f64) -> Vec3 {
        for i in 0..self.colour_vec.len() {
            if value > self.colour_vec[i].cutoff {
                return self.colour_vec[i].colour;
            }
        }
        return self.default_colour;
    }
}