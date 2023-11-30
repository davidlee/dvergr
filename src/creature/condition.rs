use crate::typical::*;
// Condition
//
#[derive(Component, Debug, Clone, Default, Eq, PartialEq)]
#[allow(dead_code)]
pub struct CreatureCondition {
    needs: (),
    conditions: (),
    injuries: (),
    encumberance: (),
}

#[allow(dead_code)]
impl CreatureCondition {
    fn default() -> Self {
        CreatureCondition {
            needs: (),
            conditions: (),
            injuries: (),
            encumberance: (),
        }
    }
}
