use crate::dice::Dice;

use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Primary {
    dexterity: u8,
    agility: u8,
    resilience: u8,
    speed: u8,
    power: u8,
    will: u8,
    intuition: u8,
    magnetism: u8,
    perception: u8,
    acuity: u8,
}
impl Primary {
    fn random() -> Primary {
        let d = Dice::default();
        Primary {
            dexterity: *d.d10(),
            agility: *d.d10(),
            resilience: *d.d10(),
            speed: *d.d10(),
            power: *d.d10(),
            will: *d.d10(),
            intuition: *d.d10(),
            magnetism: *d.d10(),
            perception: *d.d10(),
            acuity: *d.d10(),
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Secondary {
    stamina: u8,
    reflexes: u8,
    composure: u8,
    stride: f64,   // square per tick at Relaxed pace
    recovery: f64, // stamina per tick at rest
}

impl Secondary {
    fn new(attrs: &Primary) -> Self {
        Secondary {
            stamina: (attrs.resilience + attrs.power) / 2,
            reflexes: (attrs.speed + attrs.acuity) / 2,
            composure: (attrs.will + attrs.magnetism) / 2,
            recovery: 1.0,
            stride: 1.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct Attributes {
    primary: Primary,
    secondary: Secondary,
    // all this shit should be part of PlayerBundle, right?
    // stance: Stance,
    // facing: Direction,
    // tempo: Tempo,
    // current_action: (),
    // actions_queued: (),
    // needs: (),
    // conditions: (),
    // injuries: (),
    // wearing: (),
    // carrying: (),
    // anatomy: (),
    // thoughts: (),
}

impl Attributes {
    pub fn new() -> Attributes {
        let primary = Primary::random();
        let secondary = Secondary::new(&(primary.clone()));
        // let tempo = TEMPOS[0].clone();

        Attributes {
            primary,
            secondary,
            // stance: Stance::Standing,
            // facing: Direction::North,
            // tempo,
            // current_action: (),
            // actions_queued: (),
            // needs: (),
            // conditions: (),
            // injuries: (),
            // wearing: (),
            // carrying: (),
            // anatomy: (),
        }
    }
}
