// Material
use bevy::prelude::Component;

// stuff other stuff can be made of (adjective)
// eg a Cell or Item
// which affects its value, function, or appearance
// eg a Bronze Crossbow; a Dirt Cell; a Paper Scroll; a Granite Bench
//
// stuff which is only relevant as an item, liquid, etc in its own right does
// not belong here
//
// for example: water; milk; blueberries; hay;
//
// TODO:
// a Ruby Cell != a single ruby
// is there a Seam or other intermediary?
//
#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Material {
    Wood(Wood),
    Metal(Metal),
    Mineral(Mineral),
    Gem(Gem),
    Textile(Textile),
    Shell(Species),
    Leather(Species),
    // Oil(Oil),
    // Fat(Fat),
    Writing(Writing),
    Ingredient(Ingredient),
    Bone(Species),
    // includes crafting materials but not foods / ingredients
    Plant(Plant),
    Animal(Animal),
    // Refuse,
    //
    Glass,
    Dirt,
    Mud,
    Clay,
    Ash,
    Wax,
    // Slime,
    // Blood,
    // Liquid(),
}

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Wood {
    Birch,
}

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Ingredient {
    Pepper,
}

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Species {
    Human,
    Cow,
    Tortoise,
    Cowry,
    Seashell,
    Rat,
    Dwarf,
    Elf,
    Goblin,
}

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Metal {
    Iron,
    Copper,
    Tin,
    Bronze,
    Silver,
    Gold,
    Lead,
    Pewter,
    Zinc,
    Magnesium,
    Platinum,
    Mercury,
    Chromium,
}

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Mineral {
    Sandstone,
    Granite,
    Marble,
    Quartz,
    Stone,
    Sand,
    Mica,
    Pyrite,
    Basalt,
    Andesite,
    Diorite,
    Obsidian,
    Chert,
    Dolomite,
    Coal,
    Flint,
    Limestone,
    Mudstone,
    Gneiss,
    Slate,
    Clay,
    Charcoal,
    Ash,
    Salt,
}

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Gem {
    Ruby,
    Diamond,
    Sapphire,
}

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Glass {
    Green,
    Clear,
    Crystal,
}

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Textile {
    Linen,
    Leather(),
    Cotton,
    Wool(),
    Fur(),
    Silk(),
    Coir,
    Flax,
    Jute,
}

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Plant {
    Flax,
    Cotton,
    Coir,
    Bamboo,

    // TODO grains don't belong here
    Wheat,
    Oats,
    Barley,
    Corn,
}

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Animal {
    Wool(Species),
    Hair(Species),
    Teeth(Species),
    Ivory(Species),
    Horn(Species),
    Skull(Species),
    Bone(Species),
    // Corpse(CreatureType),
}

// #[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
// pub enum Wax {
//     Beeswax,
// }

// #[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
// pub enum Shell {
//     Cowry,
// }

// #[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
// pub enum Bone {
//     Cow,
// }

// #[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
// pub enum Oil {
//     Whale,
//     Fish,
//     Flaxseed,
// }

// #[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
// pub enum Fat {
//     Lard,
//     Butter,
// }

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum Writing {
    Paper,
    Vellum,
    Papyrus,
    Parchment,
    // Clay,
    // Wax,
}

#[derive(Component, Eq, PartialEq, Clone, Debug, PartialOrd, Ord, Hash)]
pub enum State {
    Solid,
    Granules,
    Paste,
    Slurry,
    Liquid,
    Magma,
    Gas,
    Energy,
}

/*
refuse
    corpses
    bones
    teeth
    hair
*/
