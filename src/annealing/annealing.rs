use crate::math_utils::*;
use crate::{HEIGHT, MID};

use rand::Rng;
use rand::seq::index::sample;

const _ITERS: u32 = 100000000;

#[derive(Debug)]
pub struct Anneal {
    pub cords: Vec<Point>,
    t: f64,
    //k: u32,
    p: f64,
    no_changes: u32,
}

impl Anneal {
    pub fn new(cords: Vec<Point>) -> Self {
        let mut p = 0.0;

        for i in 0..cords.len() - 1 {
            for j in i + 1..cords.len() {
                p += lj(sqr_dist_eu(&cords[i], &cords[j]));
            }
        }

        Anneal {
            cords,
            t: 1.0,
            //k: 0,
            p,
            no_changes: 0,
        }
    }

    pub fn should_inc(&mut self) -> bool {
        if self.no_changes > 100 {
            self.no_changes = 0;
            return true;
        } else {
            return false;
        }
    }

    pub fn step_n(&mut self, n: usize, iters: usize) {
        let mut changed = false;

        for _ in 0..iters {
            let mut rng = rand::thread_rng();

            let mut cords2 = self.cords.clone();

            let mut pot2 = 0.0;

            for i in sample(&mut rng, cords2.len(), n) {
                let xshift: f64 = rng.gen_range(-0.5..0.5);
                let yshift: f64 = rng.gen_range(-0.5..0.5);
                let zshift: f64 = rng.gen_range(-0.5..0.5);


                assert_eq!(cords2[i].0.len(), 3);
                cords2[i].0[0] += xshift;
                cords2[i].0[1] += yshift;
                cords2[i].0[2] += zshift;
            }

            for i in 0..cords2.len() - 1 {
                for j in i + 1..cords2.len() {
                    pot2 += lj(sqr_dist_eu(&cords2[i], &cords2[j]));
                }
            }

            if pot2 < self.p {
                changed = true;
                self.cords = cords2;
                self.p = pot2;
            }
        }

        if changed {
            self.no_changes = 0;
        } else {
            self.no_changes += 1;
        }
    }

    pub fn step_all(&mut self) {
        let mut changed = false;

        for _ in 1..100 {
            let mut rng = rand::thread_rng();


            let mut cords2 = self.cords.clone();
            let mut pot2 = 0.0;

            for p in cords2.iter_mut() {
                let xshift: f64 = rng.gen_range(-1.0..1.0);
                let yshift: f64 = rng.gen_range(-1.0..1.0);
                let zshift: f64 = rng.gen_range(-1.0..1.0);

                assert_eq!(p.0.len(), 3);
                p.0[0] += xshift;
                p.0[1] += yshift;
                p.0[2] += zshift;
            }

            for i in 0..cords2.len() - 1 {
                for j in i + 1..cords2.len() {
                    pot2 += lj(sqr_dist_eu(&cords2[i], &cords2[j]));
                }
            }

            if pot2 < self.p {
                self.cords = cords2;
                self.p = pot2;
                changed = true;
            }
        }

    }

    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..5000 {
            let mut i: usize = rng.gen_range(0..self.cords.len());

            let mut p2 = self.cords[i].clone();
            let xshift: f64 = rng.gen_range(-10.0..10.0);
            let yshift: f64 = rng.gen_range(-10.0..10.0);

            let mut pot2 = self.p;

            assert_eq!(p2.0.len(), 2);
            p2.0[0] += xshift;
            p2.0[1] += yshift;

            for j in 0..self.cords.len() {
                if j == i {
                    continue;
                }

                pot2 -= lj(sqr_dist_eu(&self.cords[i], &self.cords[j]));
                pot2 += lj(sqr_dist_eu(&p2, &self.cords[j]));
            }

            if pot2 < self.p {
                self.cords[i] = p2;
                self.p = pot2;
            }
        }
    }

    pub fn step_legacy(&mut self) {
        if self.t != 0.0
        /* && self.k < ITERS */
        {
            self.t *= 0.99;
            let mut pot = 0.0;

            for i in 0..self.cords.len() - 1 {
                for j in i + 1..self.cords.len() {
                    pot += lj(sqr_dist_eu(&self.cords[i], &self.cords[j]));
                }
            }

            let mut rng = rand::thread_rng();

            if switch(p(pot, self.t)) {
                for p in self.cords.iter_mut() {
                    for c in p.0.iter_mut() {
                        let mut d = rng.gen::<f64>() / 1.0;
                        if rng.gen() {
                            d *= -1.0;
                        }
                        *c += d;
                    }
                }
            }

            //self.k += 1;
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0, 0, 0, 0xff]);
        }
        for p in &self.cords {
            assert_eq!(p.0.len(), 2);
            let (x, y) = (p.0[0] as i64, p.0[1] as i64);
            let wind_d = (y * HEIGHT as i64 + x) as u32;
            frame[((MID + wind_d) * 4) as usize] = 0xff;
            frame[((MID + wind_d) * 4 + 1) as usize] = 0xff;
            frame[((MID + wind_d) * 4 + 2) as usize] = 0xff;
            frame[((MID + wind_d) * 4 + 3) as usize] = 0xff;
        }
    }
}
