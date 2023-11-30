use crate::graphics::typical::*;
use crate::typical::*;

#[derive(Component, Debug, Default)]
pub struct PlayerAvatar;

#[derive(Bundle, Debug, Default)]
pub struct PlayerAvatarBundle {
    avatar: PlayerAvatar,
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_player_avatar(
    mut commands: Commands,
    sprites: Res<DwarfSpritesheet>,
    board: Res<Board>,
    mut stage_query: Query<(Entity, &Stage)>,
    player_query: Query<(Entity, &Player)>,
    tile_map_query: Query<&TileMap>,
    mut ev_writer: EventWriter<AppInitEvent>,
) {
    let transform: Transform;
    let (player_entity, ..) = player_query.single();
    {
        let tile_map = tile_map_query.single();
        let pos: IVec3 = board
            .creature_store
            .get_pos_for(&player_entity)
            .expect("player sprite needs somewhere to go")
            .to_owned();
        transform = transform_from_tilemap_pos(tile_map, &pos);
    }

    commands
        .get_entity(stage_query.single_mut().0) // Stage entity
        .expect("no stage, no player")
        .with_children(|s| {
            s.spawn((
                PlayerAvatarBundle::default(),
                CreatureEntityRef(player_entity),
                SpriteSheetBundle {
                    texture_atlas: sprites.atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform,
                    ..default()
                },
            ));
        });

    ev_writer.send(AppInitEvent::SetAppState(AppState::Game));
}
