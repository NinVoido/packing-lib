use std::f64::consts::E;
use std::iter::zip;

use rand::random;

#[derive(Debug, Clone)]
pub struct Point(pub Vec<f64>);

impl Point {
    pub fn new(cords: Vec<f64>) -> Self {
        Point(cords)
    }

    #[allow(dead_code)]
    pub fn empty() -> Self {
        Point(Vec::new())
    }
}

/// Returns squared distance for the purpose of speed
pub fn sqr_dist_eu(a: &Point, b: &Point) -> f64 {
    let mut cum_sum = 0.0;

    for (a_c, b_c) in zip(&a.0, &b.0) {
        cum_sum += (a_c - b_c).powi(2);
    }

    return cum_sum;
}

/// Uses squared distance for the purpose of speed
#[inline(always)]
pub fn lj(r: f64) -> f64 {
    let tmp_x = 1.0 / r.powi(3);
    return tmp_x.powi(2) - 2.0 * tmp_x;
}

#[inline(always)]
pub fn p(potenc: f64, t: f64) -> f64 {
    return E.powf(-1.0 / ((potenc + 10000.0) * t));
}

#[inline(always)]
pub fn switch(prob: f64) -> bool {
    return random::<f64>() < prob;
}
