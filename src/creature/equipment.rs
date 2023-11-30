use crate::typical::*;

// Equipment
//
#[derive(Component, Debug, Clone, Default, Eq, PartialEq)]
#[allow(dead_code)]
pub struct Equipment {
    worn_armour: (),
    equipped: (),
    wearing: (),
    carrying: (),
}

#[allow(dead_code)]
impl Equipment {
    fn default() -> Self {
        Equipment {
            worn_armour: (),
            equipped: (),
            wearing: (),
            carrying: (),
        }
    }
}
