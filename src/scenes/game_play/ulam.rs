use ulam::{self, calc_coord::calc_xy, value_of_xy};

pub struct GridBlockData {
    pub x: i32,
    pub y: i32,
    pub grid_x: i32,
    pub grid_y: i32,
    pub grid_index: u32,
    pub grid_xindex: usize,
    pub grid_yindex: usize,
    pub value: u32,
}
pub fn get_25_blocks(origin: u32) -> Vec<GridBlockData> {
    let mut r_vec: Vec<GridBlockData> = Vec::with_capacity(25);
    let (x, y) = calc_xy(origin);

    let mut grid_index: u32 = 0;
    for (yindex, dy) in (-2..=2).rev().enumerate() {
        for (xindex, dx) in (-2..=2).enumerate() {
            grid_index += 1;
            let gbd = GridBlockData {
                x,
                y,
                grid_x: dx,
                grid_y: dy,
                grid_index,
                grid_xindex: xindex,
                grid_yindex: yindex,
                value: value_of_xy(x + dx, y + dy),
            };
            r_vec.push(gbd);
        }
    }

    r_vec
}
