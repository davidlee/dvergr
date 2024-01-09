use crate::anatomy::humanoid::Location;
use crate::anatomy::*;
use bevy::prelude::*;
use std::{fmt::Debug, hash::Hash};

pub mod weapons {}

/*
//
// Should I use ECS for items?
+ Using ECS makes it easy to move them between map, character, etc
+ Answers how to do composition easily
- Cumbersome to manipulate

//
// If so, what structure?

Entity (Player, Anatomy, ..)
    Entity (Position::Back)
        Entity (Carryable, Wearable, Stowable, Material, ItemDetail): Linen Shirt
        Entity (Carryable, Wearable, Stowable, Armor, Material, ItemDetail): Chain Hauberk

        Entity (Carryable, Wearable, Container, ItemDetail): Backpack
            Entity(Carryable, Stowable, LiquidContainer, LiquidType, ItemDetail): Bottle of Water
            Entity(Carryable, Stowable, Wearable, Material, ItemDetail): Wool Cap
            Entity(Carryable, Stowable, Wearable, Container, Material, ItemDetail): Leather Pouch
                Entity(Carryable, Stowable, ..): Sling Stone (9)
            Entity(Carryable, Wieldable, Stowable, Weapon, Material, .. ItemDetail): Leather Sling



*/

// we will need to store a ref to item from the slot / container
// do we also want to from the item?
// + ensures single location
// + allows bidirectional traversal
// - need to keep in sync
// - complicates management functions

// EVENTS

#[derive(Event, Debug, Copy, Clone)]
pub struct ItemPickUpEvent {
    pub owner: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
pub struct ItemDropEvent {
    pub owner: Entity,
    pub item: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
pub struct ItemStowEvent {
    pub owner: Entity,
    pub item: Entity,
    pub container: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
pub struct ItemRetrieveEvent {
    pub owner: Entity,
    pub item: Entity,
    pub container: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct ItemDonEvent {
    pub owner: Entity,
    pub item: Entity,
    pub container: Entity,
    pub locations: Vec<Location>, //Location,
}

#[derive(Event, Debug, Clone)]
pub struct ItemDoffEvent {
    pub owner: Entity,
    pub item: Entity,
    pub container: Entity,
    pub locations: Vec<Location>, //Location,
}

#[derive(Event, Debug, Clone)]
pub struct ItemEquipEvent {
    pub owner: Entity,
    pub item: Entity,
    pub hands: Side,
}

#[derive(Event, Debug, Clone)]
pub struct ItemUnequipEvent {
    pub owner: Entity,
    pub item: Entity,
    pub hands: Side,
}

#[derive(Component, Debug, Clone, Eq, PartialEq)]
pub enum ItemLocation {
    Cell(Entity, IVec3),
    Container(Entity),
    Worn(Entity, Vec<Location>),
    Hand(Entity, Side),
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ItemCategory {
    Ammunition(AmmunitionType),
    Furniture,
    Container,
    Drink,
    Liquid,
    Plant,
    Animal,
    Armor,
    Clothing,
    BarOrBlock,
    Cloth,
    Coins,
    Gem,
    Leather,
    Corpse,
    Refuse,
    Food,
    Sheets,
    Weapons,
    Tool,
    Stone,
    Wood,
    Consumable,
    Arcane,
    Plot,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AmmunitionType {
    Arrow,
    Bolt,
    Dart,
    Stone,
    // ThrowingKnife,
    // ThrowingAxe,
    Grenade,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Quality {
    Terrible,
    Poor,
    Average,
    Good,
    Superior,
    Great,
    Masterwork,
    Legendary,
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ItemDetail<ItemCategory> {
    pub category: ItemCategory,
    pub display_name: String,
    pub description: String,
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Stowable {
    pub weight: u16, // grams
    pub volume: u16, // mL
    pub length: u16, // cm
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Carryable {
    pub hands: Side,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Wieldable {
    pub hands: Side,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Usable {
    pub hands: Side,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TakesAmmo {
    pub ammo_type: (),
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Durability {
    condition: u8,
}

impl Default for Durability {
    fn default() -> Durability {
        Self { condition: 255 }
    }
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Stackable {
    pub count: u8,
    pub max: u8,
}
impl Default for Stackable {
    fn default() -> Stackable {
        Self { count: 1, max: 255 }
    }
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Valuable {
    pub base_value: u16,
    pub computed_value: u16,
}

// some containers can only contain certain items
// eg a quiver -> arrows

// furniture can only be stowed in a bin
//

#[derive(Debug, Default, Clone, Component)]

pub struct Container {
    pub contents: Vec<Entity>,
    pub volume: u16,
    pub max_length: u16,
}

// retrieving an item takes time

impl Container {
    // add -> bool
    // take
    // find(time) -> Option<item>
}

#[derive(Debug, Default, Clone, Component)]
pub struct LiquidContainer {
    pub contents: Option<Liquid>,
    pub volume: u16,
}

impl LiquidContainer {}
// impl Carryable for LiquidContainer {}

#[derive(Debug, Default, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum LiquidType {
    #[default]
    Water,
    Brine,
    Ale,
    Wine,
    Slime,
    Blood,
}

#[derive(Debug, Default, Clone, Component)]
pub struct Liquid {
    pub kind: LiquidType,
    pub volume: u16,
}

pub enum ConsumableUse {
    SingleUse,
    LimitedUse(u8),
    Fragile(f32), // random chance consumed on use; avg # of uses until consumed
}

pub enum ConsumableMethod {
    Drinkable,
    Edible,
    Poultice,
    Ointment,
    Burnt,
    Ammunition,
    Thrown,
    Tool,
    Ingredient,
    Instant, // e.g. wand
    Casting, // component
    Ritual,  //
}

#[derive(Debug, Default, Clone, Component, Eq, Hash)]
struct Consumable {
    uses: ConsumableUse,
    method: ConsumableMethod,
}

// #[derive(Component, Debug, Clone)]
// pub enum WearableSlotStatus {
//     Free,
//     Occupied, // you may be wearing a shirt, but can still layer a gambeson
//     // while full plate might allow a tabard over it, but not a ball gown
//     Covered, // you can't don glasses while wearing a helmt -
//              // but nor does it prevent you wearing them underneath
// }

// pub enum CarryableSlotStatus {
//     Free,
//     Occupied,
// }
