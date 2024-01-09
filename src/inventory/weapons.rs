pub enum WeaponType {
    CloseQuarters(CloseQuartersWeaponType),
    Personal(PersonalWeaponType),
    Missile(MissileWeaponType),
}

pub enum WeaponRange {}

pub enum CloseQuartersWeaponType {
    Dagger,
    PushKnife,
    KnuckleKnife,
    BearClaws,
    Cestus,
    BrassKnuckles,
    Gauntlet,
    Buckler,
}

pub enum PersonalWeaponType {
    Polearm,
    MassWeapon,
    Dagger,
    Club,
    Sword,
    Net,
    Lance,
    Pike,
}

pub enum SwordType {
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

pub enum LineWeaponType {
    Pike,
    Lance,
}

pub enum HaftLength {
    Short,
    Bastard,
    Double,
    Long(usize), // feet
}

pub enum MissileWeaponType {
    Bow,
    Crossbow,
    // Sling,
    Throwing,
    Thrower,
}

pub enum Bow {
    Shortbow,
    Longbow,
    Flatbow,
    RecurveBow,
    CompositeRecurveBow,
}

pub enum Crossbow {
    Hand,
    Light,
    Heavy,
    Arbalest,
}

pub enum Thrown {
    Knife,
    Axe,
    Javelin,
    Bolas,
    Net,
    Dart,
    Star,
    Grenade,
}

pub enum Thrower {
    Sling,
    DartThrower,
    Atlatl,
    Discus,
}

// combine for fun

pub enum Head {
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

// pub enum WeaponHaft {} wood vs metal

pub enum Guard {
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

pub enum Pommel {
    None,
    Blunt,
    Spiked,
}

pub enum Balance {
    Parrying,
    Thrusting,
    Cutting,
    Chopping,
    Bashing,
}

pub enum BladeType {
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
pub enum BladeCurvature {
    Straight, // rapier
    Gentle,   // katana
    Balanced, // saber
    Slicing,  // scimitar
    Recurve,
    Flambard,
}

// pub enum Polearm {
//     Staff,
//     Spear,
//     Halberd,
//     Pike,
//     Bill,
//     Glaive,
//     Trident,
// }
