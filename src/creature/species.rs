use crate::typical::*;

// Species
//
#[derive(Component, Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Default)]
#[allow(dead_code)]
pub struct Species {
    name: String,
    anatomy_template: (),
    subtype: Option<()>,
}

#[allow(dead_code)]
impl Species {
    fn default() -> Self {
        Species {
            name: String::from("human"),
            anatomy_template: (),
            subtype: None,
        }
    }

    pub fn humanoid(name: &str) -> Self {
        Species {
            name: String::from(name),
            anatomy_template: (),
            subtype: None,
        }
    }

    pub fn human() -> Self {
        Species::humanoid("human")
    }
}
