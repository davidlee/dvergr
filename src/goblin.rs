// use crate::typical::graphics::*;
use crate::typical::*;

// #[derive(Bundle, Default)]

pub(crate) fn goblin_bundle() -> CreatureBundle {
    CreatureBundle {
        creature: Creature {
            dry_weight: 55.,
            height: 125,
        },
        species: Species::Goblin,
        size: CreatureSize::Small,
        // rest
        ..default()
    }
}

fn spawn_gobbo(
    //
    mut board: ResMut<Board>,
) {
    //
}
