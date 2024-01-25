#![allow(dead_code)]
use crate::typical::*;

pub(crate) mod weapons {}

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
pub(crate) struct ItemPickUpEvent {
    pub(crate) owner: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
pub(crate) struct ItemDropEvent {
    pub(crate) owner: Entity,
    pub(crate) item: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
pub(crate) struct ItemStowEvent {
    pub(crate) owner: Entity,
    pub(crate) item: Entity,
    pub(crate) container: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
pub(crate) struct ItemRetrieveEvent {
    pub(crate) owner: Entity,
    pub(crate) item: Entity,
    pub(crate) container: Entity,
}

#[derive(Event, Debug, Clone)]
pub(crate) struct ItemDonEvent {
    pub(crate) owner: Entity,
    pub(crate) item: Entity,
    pub(crate) container: Entity,
    pub(crate) locations: Vec<Location>, //Location,
}

#[derive(Event, Debug, Clone)]
pub(crate) struct ItemDoffEvent {
    pub(crate) owner: Entity,
    pub(crate) item: Entity,
    pub(crate) container: Entity,
    pub(crate) locations: Vec<Location>, //Location,
}

#[derive(Event, Debug, Clone)]
pub(crate) struct ItemEquipEvent {
    pub(crate) owner: Entity,
    pub(crate) item: Entity,
    pub(crate) hands: Side,
}

#[derive(Event, Debug, Clone)]
pub(crate) struct ItemUnequipEvent {
    pub(crate) owner: Entity,
    pub(crate) item: Entity,
    pub(crate) hands: Side,
}

#[derive(Component, Debug, Clone, Eq, PartialEq)]
pub(crate) enum ItemLocation {
    Cell(Entity, IVec3),
    Container(Entity),
    Worn(Entity, Vec<Location>),
    Hand(Entity, Side),
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) enum ItemCategory {
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
pub(crate) enum AmmunitionType {
    Arrow,
    Bolt,
    Dart,
    Stone,
    // ThrowingKnife,
    // ThrowingAxe,
    Grenade,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) enum Quality {
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
pub(crate) struct ItemDetail<ItemCategory> {
    pub(crate) category: ItemCategory,
    pub(crate) display_name: String,
    pub(crate) description: String,
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) struct Stowable {
    pub(crate) weight: u16, // grams
    pub(crate) volume: u16, // mL
    pub(crate) length: u16, // cm
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) struct Carryable {
    pub(crate) hands: Side,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) struct Wieldable {
    pub(crate) hands: Side,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) struct Usable {
    pub(crate) hands: Side,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) struct TakesAmmo {
    pub(crate) ammo_type: (),
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) struct Durability {
    condition: u8,
}

impl Default for Durability {
    fn default() -> Durability {
        Self { condition: 255 }
    }
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Stackable {
    pub(crate) count: u8,
    pub(crate) max: u8,
}
impl Default for Stackable {
    fn default() -> Stackable {
        Self { count: 1, max: 255 }
    }
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Valuable {
    pub(crate) base_value: u16,
    pub(crate) computed_value: u16,
}

// some containers can only contain certain items
// eg a quiver -> arrows

// furniture can only be stowed in a bin
//

#[derive(Debug, Default, Clone, Component)]

pub(crate) struct Container {
    pub(crate) contents: Vec<Entity>,
    pub(crate) volume: u16,
    pub(crate) max_length: u16,
}

// retrieving an item takes time

impl Container {
    // add -> bool
    // take
    // find(time) -> Option<item>
}

#[derive(Debug, Default, Clone, Component)]
pub(crate) struct LiquidContainer {
    pub(crate) contents: Option<Liquid>,
    pub(crate) volume: u16,
}

impl LiquidContainer {}
// impl Carryable for LiquidContainer {}

#[derive(Debug, Default, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum LiquidType {
    #[default]
    Water,
    Brine,
    Ale,
    Wine,
    Slime,
    Blood,
}

#[derive(Debug, Default, Clone, Component)]
pub(crate) struct Liquid {
    pub(crate) kind: LiquidType,
    pub(crate) volume: u16,
}

#[derive(Debug, Default, Clone, Component, Eq, PartialEq, Hash)]
pub(crate) enum ConsumableUse {
    #[default]
    SingleUse,
    LimitedUse(u8),
    Fragile(u8), // random chance consumed on use; avg # of uses until consumed
}

#[derive(Debug, Default, Clone, Component, Eq, PartialEq, Hash)]
pub(crate) enum ConsumableMethod {
    #[default]
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

#[derive(Debug, Default, Clone, Component, Eq, PartialEq, Hash)]
struct Consumable {
    uses: ConsumableUse,
    method: ConsumableMethod,
}

// #[derive(Component, Debug, Clone)]
// pub(crate) enum WearableSlotStatus {
//     Free,
//     Occupied, // you may be wearing a shirt, but can still layer a gambeson
//     // while full plate might allow a tabard over it, but not a ball gown
//     Covered, // you can't don glasses while wearing a helmt -
//              // but nor does it prevent you wearing them underneath
// }

// pub(crate) enum CarryableSlotStatus {
//     Free,
//     Occupied,
// }
