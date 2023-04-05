use rand::{thread_rng, Rng, seq::SliceRandom};

use crate::vec3::Point3;



const POINTCOUNT: u32 = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}


impl Perlin {
    pub fn new() -> Perlin {
        let mut rng = thread_rng();
        let ranfloat = rng.gen::<[f64; POINTCOUNT as usize]>().to_vec();
        

        let perm_x = Perlin::generate_perm();
        let perm_y = Perlin::generate_perm();
        let perm_z = Perlin::generate_perm();
        // let ranfloat = ranfloat1.to_vec();
        Perlin { ranfloat, perm_x, perm_y, perm_z }

    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4.0 * p.x()).round() as i32 & 255) as usize;
        let j = ((4.0 * p.y()).round() as i32 & 255) as usize;
        let k = ((4.0 * p.z()).round() as i32 & 255) as usize;

        let index = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;
        self.ranfloat[index]
    }

    fn generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = (0..POINTCOUNT as i32).collect();
        p.shuffle(&mut thread_rng());
        p        
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}