use crate::typical::*;

use bevy_turborand::prelude::*;
use bevy_turborand::GlobalChaChaRng;
use bevy_turborand::RngComponent;

pub fn flicker_torches(
    mut commands: Commands,
    mut secondary_query: Query<(
        Entity,
        &TorchSecondaryLightMarker,
        &Parent,
        Option<&mut PointLight>,
    )>,
    mut global_rng: ResMut<GlobalChaChaRng>,
) {
    let mut rng = RngComponent::from(&mut global_rng);

    for x in secondary_query.iter_mut() {
        if rng.usize(0..10) < 6 {
            return;
        }

        let a = rng.usize(250..850) as f32;
        let b = rng.usize(250..850) as f32;

        let intensity = a + b;

        if let Some(mut light) = x.3 {
            light.intensity = intensity;
        } else {
            commands.entity(x.0).insert((SpotLightBundle {
                spot_light: SpotLight {
                    intensity,
                    range: 120.,
                    shadows_enabled: true,
                    color: Color::GOLD,
                    outer_angle: 1.5,
                    inner_angle: 0.2,
                    ..default()
                },
                ..default()
            },));
        }
    }
}
