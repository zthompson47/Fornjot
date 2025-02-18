use itertools::Itertools;

use crate::{
    objects::{Cycle, SurfaceVertex},
    storage::Handle,
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Cycle {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        CycleValidationError::check_half_edge_connections(self, errors);

        // We don't need to check that all half-edges are defined in the same
        // surface. We already check that they are connected by identical
        // surface vertices, so that would be redundant.
    }
}

/// [`Cycle`] validation error
#[derive(Clone, Debug, thiserror::Error)]
pub enum CycleValidationError {
    /// Half-edges are not connected
    #[error(
        "`HalfEdge`s of `Cycle` are not connected\n\
        - Front vertex of previous `HalfEdge`: {prev:#?}\n\
        - Back vertex of next `HalfEdge`: {next:#?}"
    )]
    HalfEdgeConnection {
        /// The front vertex of the previous half-edge
        prev: Handle<SurfaceVertex>,

        /// The back vertex of the next half-edge
        next: Handle<SurfaceVertex>,
    },
}

impl CycleValidationError {
    fn check_half_edge_connections(
        cycle: &Cycle,
        errors: &mut Vec<ValidationError>,
    ) {
        for (a, b) in cycle.half_edges().circular_tuple_windows() {
            let [_, prev] = a.vertices();
            let [next, _] = b.vertices();

            let prev = prev.surface_form();
            let next = next.surface_form();

            if prev.id() != next.id() {
                errors.push(
                    Self::HalfEdgeConnection {
                        prev: prev.clone(),
                        next: next.clone(),
                    }
                    .into(),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::CycleBuilder,
        objects::Cycle,
        partial::{Partial, PartialCycle, PartialObject},
        services::Services,
        validate::Validate,
    };

    #[test]
    fn cycle_half_edge_connections() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let mut cycle = PartialCycle {
                surface: Partial::from(services.objects.surfaces.xy_plane()),
                ..Default::default()
            };
            cycle.update_as_polygon_from_points([[0., 0.], [1., 0.], [0., 1.]]);
            cycle.build(&mut services.objects)
        };
        let invalid = {
            let mut half_edges = valid
                .half_edges()
                .map(|half_edge| Partial::from(half_edge.clone()))
                .collect::<Vec<_>>();

            // Sever connection between the last and first half-edge in the
            // cycle.
            {
                let first_half_edge = half_edges.first_mut().unwrap();
                let [first_vertex, _] = &mut first_half_edge.write().vertices;
                let surface_vertex = Partial::from_partial(
                    first_vertex.read().surface_form.read().clone(),
                );
                first_vertex.write().surface_form = surface_vertex;
            }

            let half_edges = half_edges
                .into_iter()
                .map(|half_edge| half_edge.build(&mut services.objects));

            Cycle::new(half_edges)
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }
}
