use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy)]
pub enum Need {
    Thirst,
    Hunger,
    Breath,
    Blood,
    Stress,
    Sleep,
    Pain,
    Medicine,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum Condition {
    Blind,
    Deaf,
    Dumb,
    Dehydrated,
    Exhausted,
    Angry,
    Dissociation,
    Despair,
    Berserk,
    Tantrum,
    Unconscious,
    Confused,
    Shock,
    Poison,
    Cold,
    Hot,
    Wet,
    Raving,
    Vomiting,
    Diarrhea,
    Dizziness,
    Nausea,
    Intoxicated,
    Seizure,
    BloodLoss,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum Injury {
    Shattered,
    Pulverised,
    Broken,
    Slashed,
    Chopped,
    Cleaved,
    Stabbed,
    Pierced,
    Bleeding,
    Dismembered,
    Disintegrated,
    Stroke,
    CardiacArrest,
}

pub mod parts {
    trait Skull {}
    trait Brain {}
    trait Face {}
    trait Eye {}
    trait Ear {}
    trait Nose {}
    trait Mouth {}
    trait Tongue {}
    trait Neck {}
    trait Torso {}
    trait Spine {}
    trait Ribcage {}
    trait Pelvis {}
    trait Hips {}
    trait Shoulder {}
    trait Abdomen {}
    trait Groin {}
    trait Limb {}
    trait Arm {}
    trait Bicep {}
    trait Elbow {}
    trait Forearm {}
    trait Wrist {}
    trait Hand {}
    trait Foot {}
    trait Finger {}
    trait Thigh {}
    trait Knee {}
    trait Shin {}
    trait Calf {}
    trait Thumb {}
    trait Toe {}
    trait Heart {}
    trait Lung {}
    trait Liver {}
    trait Spleen {}
    trait Intestine {}
    trait Artery {}
    trait Vein {}
    trait Nerve {}
    trait Bone {}
    trait Muscle {}
    trait Tendon {}

    // enum - left/right
    //

    // traits - head, chest, limb, arm, leg, extremity, hand, foot, finger, toe, ear, ...

    // carry - structs
}
