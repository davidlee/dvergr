use super::*;
// use crate::anatomy::*;
// use crate::creature::*;
// use crate::typical::*;

#[derive(Bundle, Debug, Clone)]
pub struct CharacterBundle {
    pub character: Character,

    pub approach: Approach,
    pub needs: NeedList,
    // pub blood
    // /pub stamina: Stamina,
    pub skills: SkillList,
    pub abililties: AbilityList,
    pub spells: SpellList,
    pub recipes: RecipeList,
    pub blueprints: BlueprintList,
    pub rituals: RitualList,
    // Journal
}

impl Default for CharacterBundle {
    fn default() -> Self {
        Self {
            // age: Age(16),
            ..default()
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Character {
    pub name: Option<String>,
    pub level: CharacterLevel,
    pub experience: u16,
}

#[derive(Component, Debug, Clone, Default)]
pub struct Age(pub u16);

#[derive(Component, Debug, Clone, Default)]
pub struct CharacterLevel(pub u16);

#[derive(Component, Debug, Clone, Default)]
pub struct SkillList {}

#[derive(Component, Debug, Clone, Default)]
pub struct AbilityList {}

#[derive(Component, Debug, Clone, Default)]
pub struct SpellList {}

#[derive(Component, Debug, Clone, Default)]
pub struct RecipeList {}

#[derive(Component, Debug, Clone, Default)]
pub struct BlueprintList {}

#[derive(Component, Debug, Clone, Default)]
pub struct RitualList {}
