use bevy::{math::IVec3, utils::HashSet};

// https://www.redblobgames.com/grids/circle-drawing/
//

pub fn circle_hash_set(centre: IVec3, radius: i32) -> HashSet<[i32; 3]> {
    let mut circle = HashSet::new();

    let [x, y, z] = centre.to_array();
    let [fx, fy, fr] = [x as f32, y as f32, radius as f32 + 0.5];
    let r2 = fr * fr;

    let top = fy - fr;
    let bot = fy + fr;

    for y in f32::round(top) as i32..f32::round(bot) as i32 {
        let dy = y as f32 - fy;
        let dx = f32::sqrt(r2 - dy * dy);
        let left = f32::ceil(fx - dx);
        let right = f32::floor(fx + dx);

        for x in f32::round(left) as i32..f32::round(right) as i32 {
            circle.insert([x, y, z]);
        }
    }
    circle
}
