pub mod begin_action {
    use crate::typical::*;

    #[derive(Event, Debug)]
    pub struct UpdateLocus {
        pub entity: Entity,
        pub locus: Locus,
        pub from: Position,
    }
}
