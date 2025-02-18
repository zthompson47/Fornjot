use crate::{
    objects::Cycle,
    partial::{Partial, PartialCycle, PartialFace},
};

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Add an interior cycle
    fn add_interior(&mut self) -> Partial<Cycle>;
}

impl FaceBuilder for PartialFace {
    fn add_interior(&mut self) -> Partial<Cycle> {
        let cycle = Partial::from_partial(PartialCycle {
            surface: self.exterior.read().surface.clone(),
            ..Default::default()
        });
        self.interiors.push(cycle.clone());
        cycle
    }
}
