use super::*;

#[derive(Bundle, Debug, Clone, Default)]
pub struct CharacterBundle {
    pub character: Character,
    // pub blood
    // /pub stamina: Stamina,
    pub recipes: RecipeList,
    pub blueprints: BlueprintList,
    pub rituals: RitualList,
    // Journal
}

#[derive(Component, Debug, Clone, Default)]
pub struct Character {
    pub name: Option<String>,
    pub level: CharacterLevel,
    pub experience: u16,
}

#[derive(Component, Debug, Clone, Default)]
pub struct CharacterLevel(pub u16);

#[derive(Component, Debug, Clone, Default)]
pub struct RecipeList {}

#[derive(Component, Debug, Clone, Default)]
pub struct BlueprintList {}

#[derive(Component, Debug, Clone, Default)]
pub struct RitualList {}
