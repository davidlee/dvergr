pub(crate) enum WeaponType {
    CloseQuarters(CloseQuartersWeaponType),
    Personal(PersonalWeaponType),
    Missile(MissileWeaponType),
}

pub(crate) enum WeaponRange {}

pub(crate) enum CloseQuartersWeaponType {
    Dagger,
    PushKnife,
    KnuckleKnife,
    BearClaws,
    Cestus,
    BrassKnuckles,
    Gauntlet,
    Buckler,
}

pub(crate) enum PersonalWeaponType {
    Polearm,
    MassWeapon,
    Dagger,
    Club,
    Sword,
    Net,
    Lance,
    Pike,
}

pub(crate) enum SwordType {
    Dagger,
    Dirk,
    HouseKnife,
    Stilletto,
    MainGauche,
    Messer,
    Falchion,
    ShortSword,
    Cutlass,
    SmallSword,
    Sabre,
    ArmingSword,
    Estoc,
    Scimitar,
    Longsword, // = bastard sword
    Claymore,
    Zweihander,
    Kriegsmesser,
}

pub(crate) enum LineWeaponType {
    Pike,
    Lance,
}

pub(crate) enum HaftLength {
    Short,
    Bastard,
    Double,
    Long(usize), // feet
}

pub(crate) enum MissileWeaponType {
    Bow,
    Crossbow,
    // Sling,
    Throwing,
    Thrower,
}

pub(crate) enum Bow {
    Shortbow,
    Longbow,
    Flatbow,
    RecurveBow,
    CompositeRecurveBow,
}

pub(crate) enum Crossbow {
    Hand,
    Light,
    Heavy,
    Arbalest,
}

pub(crate) enum Thrown {
    Knife,
    Axe,
    Javelin,
    Bolas,
    Net,
    Dart,
    Star,
    Grenade,
}

pub(crate) enum Thrower {
    Sling,
    DartThrower,
    Atlatl,
    Discus,
}

// combine for fun

pub(crate) enum Head {
    Axe,   // can also hook
    Blade, // sword on a pole
    Mace,
    Flange,
    Maul,   // flat thumper
    Hammer, // directional, bites into plate
    Pick,   // directional
    Spike,  //
    BarbedSpike,
    Hook,
    Beak,
    SpearPoint,
    Mancatcher,
}

// pub(crate) enum WeaponHaft {} wood vs metal

pub(crate) enum Guard {
    None,
    Tsuba,
    Cross,
    Ring,
    Bell,
    Quillions,
    Knuckle,
    Basket, // slower to draw
    Sai,
}

pub(crate) enum Pommel {
    None,
    Blunt,
    Spiked,
}

pub(crate) enum Balance {
    Parrying,
    Thrusting,
    Cutting,
    Chopping,
    Bashing,
}

pub(crate) enum BladeType {
    BackSword,
    DoubleEdged,

    MainGauche,
    SwordBreaker,

    Messer,
    Estoc,
    Rapier,
    ArmingSword,
    GreatSword,
    Saber,
    Katana,
}

// thrusts & reach = better for open duels
// slashing = better for battlefield melee; less getting stuck, defensive arcs
pub(crate) enum BladeCurvature {
    Straight, // rapier
    Gentle,   // katana
    Balanced, // saber
    Slicing,  // scimitar
    Recurve,
    Flambard,
}

// pub(crate) enum Polearm {
//     Staff,
//     Spear,
//     Halberd,
//     Pike,
//     Bill,
//     Glaive,
//     Trident,
// }
