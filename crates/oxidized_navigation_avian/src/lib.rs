
use oxidized_navigation::colliders::OxidizedCollider;

pub struct AvianCollider;

/// This is only compiled and available when the "avian" feature is enabled.
impl OxidizedCollider for AvianCollider {
    type Component = avian3d::prelude::Collider;

    fn oxidized_into_typed_shape(collider: &avian3d::prelude::Collider) -> parry3d::shape::TypedShape {
        collider.shape_scaled().as_typed_shape()
    }

    fn oxidized_compute_local_aabb(collider: &avian3d::prelude::Collider) -> parry3d::bounding_volume::Aabb {
        collider.shape_scaled().compute_local_aabb()
    }
}
