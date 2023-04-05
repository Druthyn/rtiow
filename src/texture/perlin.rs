use rand::{thread_rng, seq::SliceRandom};

use crate::vec3::{Point3, Vec3};



const POINTCOUNT: u32 = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}


impl Perlin {
    pub fn new() -> Perlin {
        let mut ranvec = vec![Vec3::zero(); POINTCOUNT as usize];
        for i in 0..ranvec.len() {
            ranvec[i] = Vec3::random_in_range(-1, 1).unit_vector()
        }


        fn generate_perm() -> Vec<i32> {
            let mut p: Vec<i32> = (0..POINTCOUNT as i32).collect();
            p.shuffle(&mut thread_rng());
            p        
        }

        let perm_x = generate_perm();
        let perm_y = generate_perm();
        let perm_z = generate_perm();
        // let ranfloat = ranfloat1.to_vec();
        Perlin { ranvec, perm_x, perm_y, perm_z }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        
        let mut c = vec![vec![vec![Vec3::zero(); 2]; 2]; 2];

        for di in [0.0, 1.0] {
            for dj in [0.0, 1.0] {
                for dk in [0.0, 1.0] {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[
                        self.perm_x[((i+di as i32) & 255) as usize] as usize ^ 
                        self.perm_y[((j+dj as i32) & 255) as usize] as usize ^ 
                        self.perm_z[((k+dk as i32) & 255) as usize] as usize
                    ]
                }
            }
        }

        let uu = u*u*(3.0-2.0*u);
        let vv = v*v*(3.0-2.0*v);
        let ww = w*w*(3.0-2.0*w);
        let mut accum = 0.0;

        for i in [0.0, 1.0] {
            for j in [0.0, 1.0] {
                for k in [0.0, 1.0] {
                    let weight_v = Vec3::new(u-i, v-j, w-k);
                    accum += (i*uu + (1.0 - i)*(1.0 - uu))
                    * (j*vv + (1.0 - j)*(1.0 - vv))
                    * (k*ww + (1.0 - k)*(1.0 - ww))
                    * c[i as usize][j as usize][k as usize].dot(&weight_v);
                }
            }    
        }
        accum
    }

    pub fn turb(&self, p: Point3, depth: Option<u32>) -> f64 {
        let depth = depth.unwrap_or(7);
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight*self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p*2.0;
        }
        accum.abs()
    }

    
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}