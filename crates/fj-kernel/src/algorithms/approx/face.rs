//! Face approximation
//!
//! See [`FaceApprox`].

use std::collections::BTreeSet;

use fj_interop::mesh::Color;
use fj_math::Point;

use crate::objects::Face;

use super::{cycle::CycleApprox, Approx, Tolerance};

impl Approx for &Face {
    type Approximation = FaceApprox;

    fn approx(self, tolerance: Tolerance) -> Self::Approximation {
        // Curved faces whose curvature is not fully defined by their edges
        // are not supported yet. For that reason, we can fully ignore `face`'s
        // `surface` field and just pass the edges to `Self::for_edges`.
        //
        // An example of a curved face that is supported, is the cylinder. Its
        // curvature is fully defined be the edges (circles) that border it. The
        // circle approximations are sufficient to triangulate the surface.
        //
        // An example of a curved face that is currently not supported, and thus
        // doesn't need to be handled here, is a sphere. A spherical face would
        // would need to provide its own approximation, as the edges that bound
        // it have nothing to do with its curvature.

        let mut points = BTreeSet::new();
        let mut exteriors = Vec::new();
        let mut interiors = BTreeSet::new();

        for cycle in self.exteriors() {
            let cycle = cycle.approx(tolerance);

            points.extend(cycle.points());
            exteriors.push(cycle);
        }
        for cycle in self.interiors() {
            let cycle = cycle.approx(tolerance);

            points.extend(cycle.points());
            interiors.insert(cycle);
        }

        // Only polygons with exactly one exterior cycle are supported.
        //
        // See this issue for some background:
        // https://github.com/hannobraun/Fornjot/issues/250
        let exterior = exteriors
            .pop()
            .expect("Can't approximate face without exterior cycle");
        assert!(
            exteriors.is_empty(),
            "Approximation only supports faces with one exterior cycle",
        );

        FaceApprox {
            points,
            exterior,
            interiors,
            color: self.color(),
        }
    }
}

/// An approximation of a [`Face`]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FaceApprox {
    /// All points that make up the approximation
    ///
    /// These could be actual vertices from the model, points that approximate
    /// an edge, or points that approximate a face.
    pub points: BTreeSet<(Point<2>, Point<3>)>,

    /// Approximation of the exterior cycle
    pub exterior: CycleApprox,

    /// Approximations of the interior cycles
    pub interiors: BTreeSet<CycleApprox>,

    /// The color of the approximated face
    pub color: Color,
}
