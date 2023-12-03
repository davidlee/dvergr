// #![allow(dead_code)]

use crate::board::CARDINAL_DIRECTIONS;
use crate::typical::*;
use crate::{BOARD_SIZE_X, BOARD_SIZE_Y};

// https://www.roguebasin.com/index.php?title=FOV_using_recursive_shadowcasting
// https://www.roguebasin.com/index.php?title=Comparative_study_of_field_of_view_algorithms_for_2D_grid_based_worlds

// https://www.albertford.com/shadowcasting/
// - https://github.com/370417/symmetric-shadowcasting/

type DepthColVec = IVec2;
type XyVec = IVec2;
type XyTuple = (i32, i32);

// pub fn mark_visible(_x: i32, _y: i32) { }
// pub fn is_wall(_x: i32, _y: i32) { }
// pub fn is_floor(_x: i32, _y: i32) { }

// fn is_floor(x: i32, y:i32)

fn scan_row_recur(
    mut row: Row,
    prev_tile: Option<DepthColVec>,
    quadrant: &Quadrant,

    walls: &HashSet<[i32; 2]>,
) -> Vec<[i32; 2]> {
    let mut prev_tile = prev_tile;
    let mut visible = Vec::new();
    let is_wall = |x, y| walls.contains(&[x, y]);
    let is_floor = |x, y| !walls.contains(&[x, y]);

    for tile in row.tiles().iter() {
        let (x, y) = quadrant.transform(tile);

        if is_wall(x, y) || is_symmetric(&row, tile) {
            visible.push([x, y]);
        }

        if prev_tile.is_some() {
            let (px, py) = quadrant.transform(&prev_tile.unwrap());

            if is_wall(px, py) && is_floor(x, y) {
                row.start_slope = Fraction::slope(tile);
            }

            if is_floor(px, py) & is_wall(x, y) {
                if let Some(mut next_row) = row.next() {
                    next_row.end_slope = slope(tile);
                    visible.append(&mut scan_row_recur(next_row, prev_tile, quadrant, walls));
                }
            }
        }
        prev_tile = Some(*tile);
    }

    if prev_tile.is_some() {
        let (px, py) = quadrant.transform(&prev_tile.unwrap());

        if is_floor(px, py) {
            let next_row = row.next();
            if next_row.is_some() {
                visible.append(&mut scan_row_recur(
                    next_row.unwrap(),
                    prev_tile,
                    quadrant,
                    walls,
                ));
            }
        }
    }
    visible
}

// TODO add bounds
pub fn compute_fov_2d_recursive<'a>(
    origin: [i32; 2],
    walls: &'a HashSet<[i32; 2]>,
) -> Vec<[i32; 2]> {
    let mut visible = vec![];
    visible.push(origin);

    for dir in CARDINAL_DIRECTIONS {
        let quadrant: Quadrant = Quadrant::new(dir, &IVec2::from_array(origin));
        let first_row = Row::new(1, Fraction::new(-1, 1), Fraction::new(-1, 1));
        visible.append(&mut scan_row_recur(first_row, None, &quadrant, walls));
    }
    visible
}
// TODO: needs to be modified to pass in functions
//
// pub fn compute_fov_2d_non_recursive(origin: XyVec, _is_blocking: bool) {
//     mark_visible(origin.x, origin.y);

//     for dir in CARDINAL_DIRECTIONS {
//         let quadrant: Quadrant = Quadrant::new(dir, origin);
//         let first_row = Row::new(1, -1.0, -1.0);
//         scan_iterative(first_row, &quadrant);
//     }
// }

// is_symmetric checks if a given floor tile can be seen symmetrically from the origin. It returns
// true if the central point of the tile is in the sector swept out by the row’s start and end slopes.
// Otherwise, it returns false.
fn is_symmetric(row: &Row, tile: &DepthColVec) -> bool {
    let [_row_depth, col] = tile.to_array();
    // FIXME check rounding
    col as i32 >= row.depth * row.start_slope.round_down()
        && col <= row.depth * row.end_slope.round_up()
}

fn slope(tile: &DepthColVec) -> Fraction {
    let [row_depth, col] = tile.to_array();
    // (2.0 * col as f32 - 1.0) / (2.0 * row_depth as f32)
    Fraction::new(2 * col - 1, 2 * row_depth)
}

// round_ties_up and round_ties_down round n to the nearest integer. If n ends in .5, round_ties_up
// rounds up and round_ties_down rounds down. Note: round_ties_up is not the same as Python’s round.
// Python’s round will round away from 0, resulting in unwanted behavior for negative numbers.
// fn round_ties_up(n: f32) -> i32 {
//     f32::floor(n + 0.5) as i32
// }

// fn round_ties_down(n: f32) -> i32 {
//     f32::ceil(n - 0.5) as i32
// }

// #[test]
// fn test_rounding() {
//     assert_eq!(round_ties_up(4.5), 5);
//     assert_eq!(round_ties_down(4.5), 4);
//     assert_eq!(round_ties_up(-4.5), -4);
//     assert_eq!(round_ties_down(-4.5), -5);
// }

pub struct Quadrant {
    pub cardinal: Direction,
    pub ox: i32,
    pub oy: i32,
}

impl Quadrant {
    pub fn transform(&self, tile: &DepthColVec) -> XyTuple {
        let [col, row] = tile.to_array();
        match self.cardinal {
            Direction::North => return (self.ox + col, self.oy - row),
            Direction::South => return (self.ox + col, self.oy + row),
            Direction::East => return (self.ox + row, self.oy + col),
            Direction::West => return (self.ox - row, self.oy + col),
            _ => panic!(),
        }
    }

    pub fn new(cardinal: Direction, origin: &XyVec) -> Self {
        let [ox, oy] = origin.to_array();
        if !CARDINAL_DIRECTIONS.contains(&cardinal) {
            panic!("illegal direction: {:?} must be cardinal", cardinal);
        }
        Quadrant { cardinal, ox, oy }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }

    pub fn slope(tile: &DepthColVec) -> Self {
        let [row_depth, col] = tile.to_array();
        Fraction::new(2 * col - 1, 2 * row_depth)
    }

    fn as_f32(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    pub fn round_up(&self) -> i32 {
        f32::floor(self.as_f32() + 0.5) as i32
    }

    pub fn round_down(&self) -> i32 {
        f32::ceil(self.as_f32() - 0.5) as i32
    }
}

#[derive(Debug, Clone)]
struct Row {
    depth: i32,
    start_slope: Fraction,
    end_slope: Fraction,
}

impl Row {
    fn new(depth: i32, start_slope: Fraction, end_slope: Fraction) -> Self {
        Row {
            depth,
            start_slope,
            end_slope,
        }
    }

    fn tiles(&self) -> Vec<DepthColVec> {
        let mut ts = vec![];
        let min_col = self.depth * self.start_slope.round_up();
        let max_col = self.depth * self.end_slope.round_down();

        if max_col > min_col && min_col > 0 {
            for col in i32::abs(min_col)..(i32::abs(max_col + 1)) {
                ts.push(IVec2::new(self.depth, col));
            }
        } else if max_col < min_col && min_col < 0 {
            for col in i32::abs(min_col)..(i32::abs(max_col + 1)) {
                ts.push(IVec2::new(self.depth, -col));
            }
        } else if max_col == min_col {
            ts.push(IVec2::new(self.depth, max_col));
        } else {
            panic!("{:?} {:?}", min_col, max_col);
        }
        warn!("[{:?} -- ({:?}] -> {:?} tiles ", min_col, max_col, ts);
        ts
    }

    fn next(&self) -> Option<Self> {
        if self.depth < 20 {
            Some(Row {
                depth: self.depth + 1,
                start_slope: self.start_slope,
                end_slope: self.end_slope,
            })
        } else {
            None
        }
    }
}

// #[test]
// fn test_range() {
//     let mut n = 0;
//     for i in 0..3 {
//         n += i;
//     }
//     assert_eq!(n, 6);

//     let mut n = 0;
//     for i in -1..-1 {
//         n += i;
//     }
// }

// non-recursive implementation of scan
// //
// fn scan_iterative(row: Row, quadrant: &Quadrant) {
//     let mut rows: Vec<Row> = vec![row];
//     while rows.len() > 0 {
//         let mut row = rows.pop().unwrap();
//         let mut prev_tile: Option<DepthColVec> = None;
//         for tile in row.tiles() {
//             if is_wall(tile, quadrant) || is_symmetric(&row, tile) {
//                 reveal(tile, quadrant);
//             }
//             if prev_tile.is_some() {
//                 if is_wall(prev_tile.unwrap(), quadrant) && is_floor(tile, quadrant) {
//                     row.start_slope = slope(tile);
//                 }
//                 if is_floor(prev_tile.unwrap(), quadrant) && is_wall(tile, quadrant) {
//                     let mut next_row = row.next().clone();
//                     next_row.end_slope = slope(tile);
//                     rows.push(next_row);
//                 }
//             }
//             prev_tile = Some(tile);
//         }
//         if is_floor(prev_tile.unwrap(), quadrant) {
//             rows.push(row.next());
//         }
//     }
// }
