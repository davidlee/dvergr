#![allow(dead_code)]

use crate::typical::*;
// Condition
//
#[derive(Component, Debug, Clone, Default, Eq, PartialEq)]
pub struct ConditionList {
    needs: (),
    conditions: (),
    injuries: (),
    encumberance: (),
}

#[allow(dead_code)]
impl ConditionList {
    fn default() -> Self {
        ConditionList {
            needs: (),
            conditions: (),
            injuries: (),
            encumberance: (),
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub enum Need {
    Recovery, // stamina
    Thirst,
    Hunger,
    Breath,
    Blood,
    Safety, // stress
    Sleep,
    Alcohol, // (substance)*
    Narcotics,
    Hope, // morale
    Greed,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum Condition {
    Blind,
    Deaf,
    Dumb,
    Exhausted,
    Hungry,
    Thirsty,
    Tired,
    Unconscious,
    Poisoned,
    Hypothermia,
    Heatstroke,
    Wet,
    Drowning,
    Choking,
    Filthy,
    Restrained,
    Entangled,
    Sick,
    Dizzy,
    Nauseous,
    Bleeding,
    Intoxicated,
    Diseased,
    Pained,
    Shocked,
    Concussed,
    Seizure,
    Stunned,
    Stressed,
    Confused,
    Surprised,
    Afraid,
    Despairing,
    Routed,
    Catatonic,
    Dead,
}

pub enum DamageType {
    Piercing,
    Slashing,
    Bludgeoning,
    // ...
    Crushing,
    Ballistic,
    Fire,
    Cold,
    Electricity,
    Acid,
    Chemical,
    Radiation,
    Divine,
    Arcane,
    Supernatural,
    Corruption,
    Psychic,
    Disease,
    Sprain,
    Trauma,
    Asphyxiation,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum Wound {
    Bruised,
    Slashed,
    Pierced,
    Fractured,
    Dislocated,
    NerveDamage,
    Crushed,
    Burned,
    Frostbitten,
    Frozen,
    Bitten,
    Stung,
    Infected,
    Transmuted,
    Torn,
    Dissolved,
    Dismembered,
    Disintegrated,
}

pub enum WoundSeverity {
    // None,
    Superficial,
    Minor,
    Major,
    Severe,
    Critical,
    Mortal,
}

// Injury Severity Score - AIS for 3 most severely injured parts of the body
