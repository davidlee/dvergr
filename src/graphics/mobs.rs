use super::typical::*;
use super::SPRITESHEET_ASSET_PATH;

use crate::typical::*;

const TILE_SIZE_W: f32 = 32.0;
const TILE_SIZE_H: f32 = 32.0;

#[derive(Resource, Debug)]
pub struct DwarfSpritesheet {
    #[allow(dead_code)]
    atlas_handle: Handle<TextureAtlas>,
}

pub fn load_spritesheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut loading: ResMut<AssetsLoading>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut ev_writer: EventWriter<AppInitEvent>,
) {
    trace!("loading SpriteSheet (characters)");

    let texture_handle: Handle<Image> = asset_server.load(SPRITESHEET_ASSET_PATH);
    let vec2 = Vec2::new(TILE_SIZE_W, TILE_SIZE_H);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle.clone(), vec2, 56, 42, None, Some(vec2));

    let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

    commands.insert_resource(DwarfSpritesheet {
        atlas_handle: texture_atlas_handle,
    });

    ev_writer.send(AppInitEvent::SetAppState(AppState::InitBoard));
}

#[derive(Component, Debug, Default)]
pub struct PlayerAvatar;

#[derive(Bundle, Debug, Default)]
pub struct PlayerAvatarBundle {
    avatar: PlayerAvatar,
}

#[derive(Component, Debug)]
pub struct CreatureEntityRef(Entity);

pub fn transform_from_tilemap_pos(tile_map: &TileMap, pos: &IVec3) -> Transform {
    let p = tile_map.tile_offset(pos.x, pos.y);

    Transform::from_xyz(
        p.x + tile_map.center_offset.x,
        p.y + tile_map.center_offset.y,
        1.0,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_player_sprite(
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

#[derive(Component, Debug)]
pub struct MobMoveAnimation {
    pub delta: Vec2,
    pub target: Vec3,
    pub frames: usize,
    pub total_frames: usize,
}

impl MobMoveAnimation {
    fn from_translation(origin: Vec3, target: Vec3, frames: usize) -> Self {
        let delta = Vec2 {
            x: (target.x - origin.x) / frames as f32,
            y: (target.y - origin.y) / frames as f32,
        };
        MobMoveAnimation {
            delta,
            frames,
            target,
            total_frames: frames,
        }
    }
}

pub fn add_changed_creature_mob_move_anim(
    mut commands: Commands,
    tile_map_query: Query<&TileMap>,
    mut sprite_query: Query<(Entity, &CreatureEntityRef, &mut Transform)>,
    changed_query: Query<(Entity, &Creature, &Locus), Changed<Locus>>,
) {
    for (_sprite_entity, CreatureEntityRef(entity), transform) in sprite_query.iter_mut() {
        if changed_query.contains(*entity) {
            let tile_map = tile_map_query.get_single().expect("WHERE IS MY TILEMAP");
            let (_, _creature, locus) = changed_query.get(*entity).unwrap();
            match locus.position {
                Position::Point(pos) => {
                    let target = transform_from_tilemap_pos(tile_map, &pos);

                    let anim = MobMoveAnimation::from_translation(
                        transform.translation,
                        target.translation,
                        6,
                    );
                    commands.entity(_sprite_entity).insert(anim);
                }
                _ => panic!("doesn't support area yet"),
            }
        }
    }
}

pub fn mob_movement(
    mut commands: Commands,
    mut sprite_query: Query<(Entity, &mut MobMoveAnimation, &mut Transform)>,
) {
    for (sprite_entity, mut anim, mut transform) in sprite_query.iter_mut() {
        if anim.frames == 1 {
            transform.translation = anim.target;
            transform.scale = Vec3::new(1.0, 1.0, 1.0);
            commands.entity(sprite_entity).remove::<MobMoveAnimation>();
        } else {
            transform.translation.x += anim.delta.x;
            transform.translation.y += anim.delta.y;
            let k = anim.total_frames as f32 / 2.0;
            let scale_factor = (k - (anim.frames as f32 - k).abs()).abs() / 35.0 + 1.0;
            transform.scale = Vec3::new(scale_factor, scale_factor, scale_factor);
            anim.frames -= 1;
        }
    }
}
