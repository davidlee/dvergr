#![allow(dead_code)]
use bevy::prelude::Component;
use std::{fmt::Debug, hash::Hash};

// use crate::typical::*;

// We treat inventory as well as anatomy here

// an injury has a
//
// location, wound, severity, treatments, conditions
// wounded, treated at ..
//

// https://github.com/veloren/veloren/blob/master/common/src/comp/body/item_drop.rs

// pub mod morphology {/

// usu. bilateral symmetry, as in humans

#[derive(Component, Eq, PartialEq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum APSymmetry {
    Back,
    Front,
    Dorsal, // e.g. spine
    None,
}

#[derive(Component, Eq, PartialEq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Side {
    Left,
    Right,
    Either,       // 1h
    Both,         // 2h
    EitherOrBoth, // weapon handedness: 1h or 2h
    Singular,     // you only have one, e.g. heart
}

#[derive(Component, Eq, PartialEq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Gender {
    Male,
    Female,
    Neither,
    Other,
}

#[derive(Component, Eq, PartialEq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum System {
    Cardiovascular,
    Respiratory,
    Endocrine,
    ImmuneLymphatic,
    Digestive,
    Muscular,
    Skeletal,
    Nervous,
    Integumentary,
    Renal,
    Reproductive,
}

/* Players / Characters have a
 - size
 - gender
 - age
 - species
 - name
 - list of skills
     - progression
 - list of talents
 - list of proficiencies
 - list of arcana
 - level
 - experience points

 - pace
 - stance
 - ? movement state

 - command_in_progress
 - command_queue

 - list of attributes
 - locus & position
 - appearance / sprite
 - computed night acclimation
 - blood alcohol level

 - list of conditions (overall)
 - list of Needs
 - litres of blood
 - stamina
 - mental fortitude / steel
 - computed morale
 - list of contextual actions available (due to equipment, etc)
 - list of contextual actions available (due to location)
 - list of faction relations
 - list of individual relationships
 - list of thoughts / moodlets

 - list of body parts
    - Location
    - WearablSlotStatus
    - Injuries
    - Treatments
    - Worn items (Vec<Location>)
        - containers ...
    - Natural Armour
    - Natural Weapons
    - Computed Armour
 - hands (Vec<Location>)
    - 2 x CarryableSlotStatus
    - carried items (Vec<Location>)

*/
pub mod humanoid {
    use super::Side;
    use bevy::prelude::Component;

    #[derive(Component, Eq, PartialEq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
    pub enum Area {
        Head,
        Trunk,
        Limbs,
    }
    #[derive(Component, Eq, PartialEq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
    pub enum Finger {
        Thumb(Side),
        Index(Side),
        Middle(Side),
        Ring(Side),
        Little(Side),
    }

    // we'll want to map this to body parts somehow
    // as well as using for inventory slots
    #[derive(Component, Eq, PartialEq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
    pub enum Location {
        Head,
        Face,
        Neck,
        Chest,
        Back,
        Abdomen,
        Groin,
        Shoulder(Side),
        UpperArm(Side),
        Elbow(Side),
        Forearm(Side),
        Wrist(Side),
        Hand(Side),
        Finger(Finger),
        Hip(Side),
        Thigh(Side),
        Knee(Side),
        Shin(Side),
        Ankle(Side),
        Foot(Side),
    }

    // #[derive(Component, Eq, PartialEq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
    // pub struct CarryLocation(Location::Hand(Side));

    #[derive(Component, Eq, PartialEq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
    pub enum Organ {
        // ...
    }

    #[derive(Component, Eq, PartialEq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
    pub enum Bone {
        // ...
    }
}
/*
- head
    - skull
        - brain
    - face
        - forehad
        - jaw
        - nose
        - cheek
        - chin
        - mouth
            - teeth
            - tongue
        - neck
            - throat
                - larynx
                - trachea
            - spine
- trunk
    - collarbone
    - shoulders
        - shoulderblade
    - ribcage
        - heart
        - lungs
        - esophagus
        - diaphragm
- abdomen
    - liver
    - gall bladder
    - stomach
    - intestines
    - spleen
    - pancreas
    - kidneys
    - colon
- hips

- groin
- thigh
- knee
- shin
- anke



*/

// pub enum Bones {
//     Skull,
//     Teeth,
//     Vertebrae,
//     Spine,
//     Ribcage,
//     Shoulderblade,
// ...
// }

// head contains eyes, mouth, brain, nose, ears
// head is an equipable location
// so are ears, eyes

// arms are composed of biceps, elbows, forearms, wrists, hands
// hands are composed of palms, fingers and thumb

// some bits (internal organs) are only interesting when damaged or status effects apply
// most of these are inside the torso, other than brain
// but limbs / general locations can also be damaged

// "equipment slots" are a thing. a sling might go over a shoulder, on a back, around a neck ...
// a sheath could be strapped to a belt, an ankle, a forearm, ...
// we could have items list the body parts / slots they can occupy, and whether they're exclusive
// (eg. plate mail is an exclusive outside layer, but shirts, or mail, or padding can be worn under)

// rings can go on fingers .. any finger (?)
// a tunic covers the thighs as well as torso
// a shirt may be long or short

// you can wear a helmet over earrings, and over glasses, but not the other way around.
// you can wear a breastplate over chain, and chain over a gambeson, and a gambeson over a shirt -
// but not the other way around.
// you can't change your undies without removing your trousers.
// but you can take off your knickers without removing your dress (unnecessary detail, perhaps)

// HIT LOCATIONS
//
// some body parts are harder to hit (a foot is smaller than a leg)
// some are more likely because of the context (hands often get hurt in swordfights)

// INVENTORY
//
// Ultimately everything carried must connect to the anatomy
// either:
//  carried (a hand / both hands / armload / shoulders - a polearm; a body; a pile of firewood)
//  fastened (a belt; a harness; a suit of armour; a ball gown; earrings)
//  worn (shirt; glove; ring; )
//  slung (backpack; bandolier; shield)
//  or stowed (inside, or attached to, something else)
//
// opening, finding, picking up, donning things all take time
// dropping things is free
//
// there is no "inventory" - only what is worn and carried.
// you can choose a "default container" to collect items into (if they will fit), or you specify where they will go.

// ENCUMBRANCE
//
// encumbrance is a function of volume, weight, shape/length, and balance / clumsiness
// there are solids, liquids, and stuff like clothes that conform to space
//   the last category can just get a discounted volume
//
// some things are long
// some things are big
// some things are heavy
// some things are unwieldy (a body; a loose sack of rocks; chain mail)
// some things are tiny (coins, rings) - finding these is harder
// some things are aggregates (a pouch of coins)
// and then there are liquids (which need containers, and leave residues)
//
// rather than recording weight, volume, and width/height/depth for each object, should we use archetypes?
// it's roughly like a coin / book / shirt / sword / cabbage / bottle / arrow / staff / basket / barrel / ...
//
// containers have a max volume, and ... ?
//  specific dimensions?
//  max weight?
//  ...

// pub struct Humanoid {}
// }

// pub mod parts {
//     trait Skull {}
//     trait Brain {}
//     trait Face {}
//     trait Eye {}
//     trait Ear {}
//     trait Nose {}
//     trait Mouth {}
//     trait Tongue {}
//     trait Neck {}
//     trait Torso {}
//     trait Spine {}
//     trait Ribcage {}
//     trait Pelvis {}
//     trait Hips {}
//     trait Shoulder {}
//     trait Abdomen {}
//     trait Groin {}
//     trait Limb {}
//     trait Arm {}
//     trait Bicep {}
//     trait Elbow {}
//     trait Forearm {}
//     trait Wrist {}
//     trait Hand {}
//     trait Foot {}
//     trait Finger {}
//     trait Thigh {}
//     trait Knee {}
//     trait Shin {}
//     trait Calf {}
//     trait Thumb {}
//     trait Toe {}
//     trait Heart {}
//     trait Lung {}
//     trait Liver {}
//     trait Spleen {}
//     trait Intestine {}
//     trait Artery {}
//     trait Vein {}
//     trait Nerve {}
//     trait Bone {}
//     trait Muscle {}
//     trait Tendon {}

// enum - left/right
//

// traits - head, chest, limb, arm, leg, extremity, hand, foot, finger, toe, ear, ...

// carry - structs
// }
