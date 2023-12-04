pub fn compute_fov_2d_recursive<'a>(
    origin: [i32; 2],
    walls: &'a HashSet<[i32; 2]>,
) -> Vec<[i32; 2]> {
    let mut visible = vec![];
    visible.push(origin);

    for cardinal in CARDINALS {
        let quadrant: Quadrant = Quadrant::new(cardinal, &IVec2::from_array(origin));
        let first_row = Row::new(1, -1.0, 1.0);
        visible.append(&mut scan_row_recur(
            first_row,
            IVec2::from_array(origin), // does this work?
            &quadrant,
            walls,
        ));
    }
    visible
}

fn scan_row_recur(
    mut row: Row,
    mut prev_tile: DepthColVec,
    quadrant: &Quadrant,
    walls: &HashSet<[i32; 2]>,
) -> Vec<[i32; 2]> {
    let mut visible = Vec::new();
    let is_wall = |x, y| walls.contains(&[x, y]);
    let is_floor = |x, y| !walls.contains(&[x, y]);

    for tile in row.tiles().iter() {
        let (x, y) = quadrant.transform(tile);
        let (prev_x, prev_y) = quadrant.transform(&prev_tile);
        if out_of_bounds(x, y) {
            return visible;
        }

        if is_wall(x, y) || is_symmetric(&row, tile) {
            visible.push([x, y]);
        }

        if is_wall(prev_x, prev_y) && is_floor(x, y) {
            row.start_slope = slope(tile);
        }

        if is_floor(prev_x, prev_y) && is_wall(x, y) {
            let mut next_row = row.next();
            next_row.end_slope = slope(tile);
            visible.append(&mut scan_row_recur(next_row, prev_tile, quadrant, walls));
        }
        prev_tile = *tile;
    }

    let (px, py) = quadrant.transform(&prev_tile);

    if is_floor(px, py) {
        let next_row = row.next();
        visible.append(&mut scan_row_recur(next_row, prev_tile, quadrant, walls));
    }
    visible
}
