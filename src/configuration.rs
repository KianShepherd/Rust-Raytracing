use crate::vec3::Vec3;
use ron::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RaytracerSettings {
    // General Settings
    pub multithreading: bool,
    pub test_scene: bool,
    // Image Settings
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    // ray Settings
    pub samples_per_pixel: usize,
    pub max_depth: i32,
    // Camera Settings
    pub v_fov: f64,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub v_up: Vec3,
    pub focal_distance: f64,
    pub aperture: f64,
    // Terrain Settings
    pub terrain_size: f64,
    pub terrain_resolution: usize,
    pub height_scale: f64,
    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub seed: u32,
}

#[allow(dead_code)]
impl RaytracerSettings {
    pub fn to_ron(&self) -> String {
        let pretty = PrettyConfig::new();

        to_string_pretty(&self, pretty).expect("serialization failed")
    }

    pub fn from_ron(ron_string: String) -> RaytracerSettings {
        from_str(&ron_string).expect("deserialization failed")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaytracerScene {
    pub multithreading: bool,
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel: usize,
    pub max_depth: i32,
    pub v_fov: f64,
    pub aperture: f64,
    pub focal_distance: f64,
    pub camera_pos: Vec<f64>,
    pub camera_dir: Vec<f64>,
    pub camera_up: Vec<f64>,
    pub objects: Vec<RonObject>,
    pub lights: Vec<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RonObject {
    objtype: String,
    vectors: Vec<Vec<f64>>,
    scalars: Vec<f64>,
    material: Vec<String>,
}

#[allow(dead_code)]
impl RaytracerScene {
    pub fn to_ron(&self) -> String {
        let pretty = PrettyConfig::new();

        to_string_pretty(&self, pretty).expect("serialization failed")
    }

    pub fn from_ron(ron_string: String) -> RaytracerScene {
        from_str(&ron_string).expect("deserialization failed")
    }
}
