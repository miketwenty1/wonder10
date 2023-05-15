use ulam::{self, calc_coord::calc_xy, value_of_xy};

pub fn get_25_blocks(origin: u32) -> Vec<String> {
    let mut r_vec: Vec<String> = Vec::with_capacity(25);
    let (x, y) = calc_xy(origin);

    for dy in (-2..=2).rev() {
        for dx in -2..=2 {
            r_vec.push(value_of_xy(x + dx, y + dy).to_string());
        }
    }

    r_vec
}
