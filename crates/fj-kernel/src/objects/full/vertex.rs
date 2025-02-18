use fj_math::Point;

use crate::{
    objects::{Curve, Surface},
    storage::Handle,
};

/// A vertex
///
/// `Vertex` is defined in terms of a 1-dimensional position on a curve. Each
/// `Vertex` has an associated [`SurfaceVertex`], which defines the 2D position
/// on the surface, and a [`GlobalVertex`] which defines the global 3D position.
///
/// Both can be accessed through [`Vertex::surface_form`].
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vertex {
    position: Point<1>,
    curve: Handle<Curve>,
    surface_form: Handle<SurfaceVertex>,
}

impl Vertex {
    /// Construct an instance of `Vertex`
    pub fn new(
        position: impl Into<Point<1>>,
        curve: Handle<Curve>,
        surface_form: Handle<SurfaceVertex>,
    ) -> Self {
        let position = position.into();

        Self {
            position,
            curve,
            surface_form,
        }
    }

    /// Access the position of the vertex on the curve
    pub fn position(&self) -> Point<1> {
        self.position
    }

    /// Access the curve that the vertex is defined in
    pub fn curve(&self) -> &Handle<Curve> {
        &self.curve
    }

    /// Access the surface form of this vertex
    pub fn surface_form(&self) -> &Handle<SurfaceVertex> {
        &self.surface_form
    }
}

/// A vertex, defined in surface (2D) coordinates
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SurfaceVertex {
    position: Point<2>,
    surface: Handle<Surface>,
    global_form: Handle<GlobalVertex>,
}

impl SurfaceVertex {
    /// Construct a new instance of `SurfaceVertex`
    pub fn new(
        position: impl Into<Point<2>>,
        surface: Handle<Surface>,
        global_form: Handle<GlobalVertex>,
    ) -> Self {
        let position = position.into();
        Self {
            position,
            surface,
            global_form,
        }
    }

    /// Access the position of the vertex on the surface
    pub fn position(&self) -> Point<2> {
        self.position
    }

    /// Access the surface that the vertex is defined in
    pub fn surface(&self) -> &Handle<Surface> {
        &self.surface
    }

    /// Access the global form of the vertex
    pub fn global_form(&self) -> &Handle<GlobalVertex> {
        &self.global_form
    }
}

/// A vertex, defined in global (3D) coordinates
///
/// This struct exists to distinguish between vertices and points at the type
/// level. This is a relevant distinction, as vertices are part of a shape that
/// help define its topology.
///
/// Points, on the other hand, might be used to approximate a shape for various
/// purposes, without presenting any deeper truth about the shape's structure.
///
/// # Validation
///
/// Vertices must be unique within a shape, meaning an identical vertex must not
/// exist in the same shape. In the context of vertex uniqueness, points that
/// are close to each other are considered identical. The minimum distance
/// between distinct vertices can be configured using the respective field in
/// [`ValidationConfig`].
///
/// [`ValidationConfig`]: crate::validate::ValidationConfig
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalVertex {
    position: Point<3>,
}

impl GlobalVertex {
    /// Construct a `GlobalVertex` from a position
    pub fn new(position: impl Into<Point<3>>) -> Self {
        let position = position.into();
        Self { position }
    }

    /// Access the position of the vertex
    pub fn position(&self) -> Point<3> {
        self.position
    }
}
