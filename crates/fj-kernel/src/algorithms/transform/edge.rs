use fj_interop::ext::ArrayExt;
use fj_math::Transform;

use crate::{
    objects::{Curve, Objects},
    partial::{MaybePartial, PartialGlobalEdge, PartialHalfEdge},
    validate::ValidationError,
};

use super::TransformObject;

impl TransformObject for PartialHalfEdge {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let surface = self
            .surface
            .map(|surface| surface.transform(transform, objects))
            .transpose()?;
        let curve = self
            .curve
            .clone()
            .map(|curve| -> Result<_, ValidationError> {
                Ok(curve
                    .into_partial()
                    .transform(transform, objects)?
                    .with_surface(surface.clone())
                    .into())
            })
            .transpose()?;
        let vertices = self.vertices.clone().try_map_ext(
            |vertex| -> Result<_, ValidationError> {
                Ok(vertex
                    .into_partial()
                    .transform(transform, objects)?
                    .with_curve(curve.clone())
                    .into())
            },
        )?;
        let global_form =
            self.global_form
                .into_partial()
                .transform(transform, objects)?
                .with_curve(curve.as_ref().and_then(
                    |curve: &MaybePartial<Curve>| curve.global_form(),
                ))
                .into();

        Ok(Self {
            surface,
            curve,
            vertices,
            global_form,
        })
    }
}

impl TransformObject for PartialGlobalEdge {
    fn transform(
        self,
        transform: &Transform,
        objects: &Objects,
    ) -> Result<Self, ValidationError> {
        let curve = self
            .curve
            .map(|curve| curve.0.transform(transform, objects))
            .transpose()?;
        let vertices = self
            .vertices
            .map(|vertices| {
                vertices.try_map_ext(|vertex| -> Result<_, ValidationError> {
                    vertex.transform(transform, objects)
                })
            })
            .transpose()?;

        Ok(Self {
            curve: curve.map(Into::into),
            vertices,
        })
    }
}
