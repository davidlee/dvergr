// #![allow(dead_code)]

use crate::typical::*;
#[allow(unused_imports)]
use crate::{BOARD_SIZE_X, BOARD_SIZE_Y};
use num_rational::*;

type F = Ratio<i32>;

type DepthColVec = IVec2;
type XyVec = IVec2;

pub fn shadowcast_visibility_2d(
    origin: [i32; 2],
    walls: &HashSet<[i32; 2]>,
) -> Vec<[i32; 2]> {
    let mut visible = vec![];
    visible.push(origin);

    for cardinal in CARDINALS {
        let quadrant: Quadrant = Quadrant::new(cardinal, &IVec2::from_array(origin));
        let first_row = Row::new(1, F::from_integer(-1), F::from_integer(1));
        visible.append(&mut scan_rows(
            first_row,
            IVec2::from_array(origin),
            &quadrant,
            walls,
        ));
    }
    visible
}
fn scan_rows(
    row: Row,
    mut prev_tile: DepthColVec,
    quadrant: &Quadrant,
    walls: &HashSet<[i32; 2]>,
    // gizmos: Gizmos,
    // query: Query<&TileMap>,
) -> Vec<[i32; 2]> {
    let mut visible = Vec::new();
    let is_wall = |x, y| walls.contains(&[x, y]);
    let is_floor = |x, y| !walls.contains(&[x, y]);
    let mut rows: Vec<Row> = vec![row];

    // if let Some(tile_map) = query.get_single() {
    //     for wall in walls {
    //         let [x,y,x] = tile_map.
    //         gizmos.ray(Vec3::from_array(x,y,z))
    //     }
    // }

    while !rows.is_empty() {
        // check if all tiles are out of bounds
        let mut oob = true;
        let mut row = rows.pop().unwrap();

        for tile in row.tiles().iter() {
            let (x, y) = quadrant.transform(tile);
            let (prev_x, prev_y) = quadrant.transform(&prev_tile);

            if oob {
                oob &= out_of_bounds(x, y); // only check if we haven't seen a valid tile in this row
            }

            // FIXME stop x-ray vision through certain walls
            // ?? check if the tile between this + origin is a wall?
            /*    ***** <-- should not be visible
            ######****# <-- should be visible
            # @       #
            #         ###

            */
            // is_wall(x, y) && (outer edge visible / adjacent to floor
            if is_wall(x, y) || is_symmetric(&row, tile) {
                // if is_symmetric(&row, tile) || (is_wall(x, y) && is_floor(prev_x, prev_y)) {
                visible.push([x, y]);
            }

            if is_wall(prev_x, prev_y) && is_floor(x, y) {
                row.start_slope = slope(tile);
            }

            if is_floor(prev_x, prev_y) && is_wall(x, y) {
                let mut next_row = row.next();
                next_row.end_slope = slope(tile);
                rows.push(next_row);
            }
            prev_tile = *tile;
        }
        let (prev_x, prev_y) = quadrant.transform(&prev_tile);
        if is_floor(prev_x, prev_y) && !oob {
            let next_row = row.next();
            rows.push(next_row);
        }
    }
    visible
}

fn out_of_bounds(x: i32, y: i32) -> bool {
    x < 0 || y < 0 || x > BOARD_SIZE_X || y > BOARD_SIZE_Y
}

// checks if a given floor tile can be seen symmetrically from the origin. It returns
// true if the central point of the tile is in the sector swept out by the row’s start and end slopes.
// Otherwise, it returns false.
fn is_symmetric(row: &Row, tile: &DepthColVec) -> bool {
    let col = tile.to_array()[1];
    let depth = row.depth;
    col >= (F::from_integer(depth) * row.start_slope).to_integer()
        && col <= (F::from_integer(depth) * row.end_slope).to_integer()
}

fn slope(tile: &DepthColVec) -> F {
    let [row_depth, col] = tile.to_array();
    F::new(2 * col - 1, 2 * row_depth)
}

// round n to the nearest integer, with choice of up or down for halway values.
// round() will round away from 0, resulting in unwanted behavior for negative numbers.

fn round_fraction_up(n: F) -> i32 {
    let f = n + F::new(1, 2);
    f.floor().to_integer()
}

fn round_fraction_down(n: F) -> i32 {
    let f = n - F::new(1, 2);
    f.ceil().to_integer()
}

struct Quadrant {
    pub cardinal: Cardinal,
    pub origin_x: i32,
    pub origin_y: i32,
}

impl Quadrant {
    // map rows & columns to global board position (x,y)
    pub fn transform(&self, tile: &DepthColVec) -> (i32, i32) {
        let [row, col] = tile.to_array();
        match self.cardinal {
            North => (self.origin_x + col, self.origin_y - row),
            South => (self.origin_x + col, self.origin_y + row),
            East => (self.origin_x + row, self.origin_y + col),
            West => (self.origin_x - row, self.origin_y + col),
        }
    }

    pub fn new(cardinal: Cardinal, origin: &XyVec) -> Self {
        let [ox, oy] = origin.to_array();
        Quadrant {
            cardinal,
            origin_x: ox,
            origin_y: oy,
        }
    }
}

#[derive(Debug, Clone)]
struct Row {
    depth: i32,
    start_slope: F,
    end_slope: F,
}

impl Row {
    fn new(depth: i32, start_slope: F, end_slope: F) -> Self {
        Row {
            depth,
            start_slope,
            end_slope,
        }
    }

    fn tiles(&self) -> Vec<DepthColVec> {
        let min_col = round_fraction_up(self.depth_fr() * self.start_slope);
        let max_col = round_fraction_down(self.depth_fr() * self.end_slope);

        (min_col..=max_col)
            .map(|col| IVec2::new(self.depth, col))
            .collect()
    }

    fn depth_fr(&self) -> F {
        F::from_integer(self.depth)
    }

    fn next(&self) -> Self {
        Row {
            depth: self.depth + 1,
            start_slope: self.start_slope,
            end_slope: self.end_slope,
        }
    }
}

enum Cardinal {
    North,
    East,
    South,
    West,
}
use Cardinal::*;

const CARDINALS: [Cardinal; 4] = [North, East, South, West];
