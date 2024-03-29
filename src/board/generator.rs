use crate::material::Substance;
use crate::player::SpawnPlayerEvent;
use crate::typical::*;

type CoOrdinate = [i32; 2];

// #[derive(Component)]
// struct PlayerPositionMarker(IVec3);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Room {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    doors: Vec<CoOrdinate>,
}

impl Ord for Room {
    fn cmp(&self, other: &Self) -> Ordering {
        [self.x, self.y].cmp(&[other.x, other.y])
    }
}

impl PartialOrd for Room {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // [self.x, self.y].partial_cmp(&[other.x, other.y])
        Some(self.cmp(other))
    }
}

const EDGE: i32 = 2;
const MARGIN: i32 = 2;

impl Room {
    fn random(rng: &mut RngComponent) -> Self {
        let width = rng.i32(6..12);
        let height = rng.i32(4..6);
        let x = rng.i32(EDGE..(BOARD_SIZE_X - width - EDGE));
        let y = rng.i32(EDGE..(BOARD_SIZE_Y - height - EDGE));
        Room {
            x,
            y,
            width,
            height,
            doors: vec![],
        }
    }

    fn max_x(&self) -> i32 {
        self.x + self.width
    }

    fn max_y(&self) -> i32 {
        self.y + self.height
    }

    fn mid_x(&self) -> i32 {
        self.x + (self.width / 2)
    }

    fn mid_y(&self) -> i32 {
        self.y + (self.height / 2)
    }
}

fn collision_free(room: &Room, rooms: &[Room]) -> bool {
    rooms.iter().all(|r| {
        (r.max_x() + MARGIN) < room.x
            || (r.max_y() + MARGIN) < room.y
            || (room.max_x() + MARGIN) < r.x
            || (room.max_y() + MARGIN) < r.y
    })
}

fn carve_room(room: &Room, blanks: &mut Vec<CoOrdinate>) {
    for y in room.y..=room.max_y() {
        for x in room.x..=room.max_x() {
            blanks.push([x, y]);
        }
    }
}

fn shared_x(rs: [&Room; 2]) -> Option<i32> {
    let ax = rs[0].x..rs[0].max_x();
    let bx = rs[1].x..rs[1].max_x();
    ax.into_iter().find(|x| bx.contains(x))
}

fn shared_y(rs: [&Room; 2]) -> Option<i32> {
    let ay = rs[0].y..rs[0].max_y();
    let by = rs[1].y..rs[1].max_y();
    ay.into_iter().find(|y| by.contains(y))
}

// where the other room lines up along either the x or y axis, we can draw the corridor
// without any turns, as long as they share that coordinate and the walls face each other;
// otherwise, we need 1 or 2 - depending on whether we use the facing wall (2), or one
// pointing sideways in the correct direction (1)

fn carve_corridors(blanks: &mut Vec<CoOrdinate>, room_a: &Room, room_b: &Room) {
    // let [facing_a, facing_b] = connected_room_facing(room_a, room_b);

    let mut rx = [room_a, room_b];
    rx.sort_by(|a, b| a.x.cmp(&b.x));

    let mut ry = [room_a, room_b];
    ry.sort_by(|a, b| a.y.cmp(&b.y));

    let (rx, ry) = (rx, ry);

    let sx = shared_x(rx);
    let sy = shared_y(ry);

    // straight connection along x axis
    if let Some(x) = sx {
        for y in ry[0].y..ry[1].max_y() {
            blanks.push([x, y]);
        }
    // straight connection along y axis
    } else if let Some(y) = sy {
        for x in rx[0].x..rx[1].max_x() {
            blanks.push([x, y]);
        }
    } else {
        let x_dist = i32::abs(rx[0].x - rx[1].x);
        let y_dist = i32::abs(ry[0].y - ry[1].y);

        // dogleg along shorter axis
        if x_dist > y_dist {
            let x_mid = rx[0].max_x() + x_dist / 2;
            for x in rx[0].max_x()..rx[1].x {
                match x {
                    _ if x < x_mid => blanks.push([x, rx[0].mid_y()]),
                    _ if x == x_mid => {
                        for y in ry[0].mid_y()..ry[1].mid_y() {
                            blanks.push([x, y]);
                        }
                    }
                    _ => blanks.push([x, rx[1].mid_y()]),
                }
            }
        } else {
            let y_mid = ry[0].max_y() + y_dist / 2;
            for y in ry[0].max_y()..ry[1].y {
                match y {
                    _ if y < y_mid => blanks.push([ry[0].mid_x(), y]),
                    _ if y == y_mid => {
                        for x in rx[0].mid_x()..rx[1].mid_x() {
                            blanks.push([x, y]);
                        }
                    }
                    _ => blanks.push([ry[1].mid_x(), y]),
                }
            }
        }
    }
}

pub(crate) fn populate_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut ev_writer: EventWriter<SpawnPlayerEvent>,
    mut global_rng: ResMut<GlobalChaChaRng>,
) {
    let mut rng = RngComponent::from(&mut global_rng);
    let mut rooms: Vec<Room> = vec![];
    let mut blanks: Vec<CoOrdinate> = vec![];
    let mut retries: usize = 0;
    let target_rooms: usize = rng.usize(5..10);

    while rooms.len() < target_rooms && retries < 1024 {
        let room = Room::random(&mut rng);
        if collision_free(&room, &rooms) {
            rooms.push(room);
        } else {
            retries += 1;
        }
    }

    rooms.sort();

    // place Player in first room
    let fst = rooms.first().expect("can't play without a player ...");
    let initial = IVec3::new(fst.x + 1, fst.y + 1, 0);
    ev_writer.send(SpawnPlayerEvent(initial));

    // determine where corridors & doors go
    let mut prev_room: Option<Room> = None;
    for room in rooms {
        if let Some(prev) = prev_room {
            carve_corridors(&mut blanks, &room, &prev);
        }
        carve_room(&room, &mut blanks);
        prev_room = Some(room);
    }

    // create cells
    commands.spawn(BoardMarker).with_children(|childer| {
        childer.spawn(ChunkMarker).with_children(|chunk| {
            for pos in board.coords().iter() {
                let [x, y, z] = pos.to_array();
                let cell = Cell::new(x, y, z);
                let floor = Floor::new(x, y, z, Substance::Dirt);

                let entity: Entity;
                if blanks.contains(&[x, y]) {
                    entity = chunk.spawn((cell, floor)).id();
                    board.floor_store.set(*pos, entity);
                    false
                } else {
                    let wall = Wall::new(x, y, z, Substance::Dirt);
                    entity = chunk.spawn((cell, floor, wall)).id();
                    board.wall_store.set(*pos, entity);
                    true
                };

                board.cell_store.set(*pos, entity);
            }
        });
    });
}

pub(crate) fn add_fun(
    board: ResMut<Board>,
    mut ev_writer: EventWriter<SpawnGoblinEvent>,
    mut global_rng: ResMut<GlobalChaChaRng>,
) {
    let mut gobbos_left = 3;
    let mut rng = RngComponent::from(&mut global_rng);
    let mut coords = board.coords();
    // randomize order
    coords.sort_unstable_by(|_a, _b| rng.u8(0..1).cmp(&1));

    while gobbos_left > 0 {
        let pos = coords.pop().unwrap();
        if board.is_unoccupied(&pos) {
            ev_writer.send(SpawnGoblinEvent(pos));
            gobbos_left -= 1;
        }
    }
}
