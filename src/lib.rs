pub mod annealing;
pub mod load_utils;
pub mod math_utils;

pub const HEIGHT: u32 = 200;
pub const WIDTH: u32 = 200;
pub const MID: u32 = (HEIGHT / 2) * WIDTH + WIDTH / 2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
