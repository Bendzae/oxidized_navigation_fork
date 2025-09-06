
use oxidized_navigation::colliders::OxidizedCollider;

pub struct RapierCollider;

/// This is only compiled and available when the "rapier" feature is enabled.
impl OxidizedCollider for RapierCollider {
    type Component = bevy_rapier3d::prelude::Collider;

    fn oxidized_into_typed_shape(collider: &bevy_rapier3d::prelude::Collider) -> parry3d::shape::TypedShape {
        collider.raw.as_typed_shape()
    }

    fn oxidized_compute_local_aabb(collider: &bevy_rapier3d::prelude::Collider) -> parry3d::bounding_volume::Aabb {
        collider.raw.compute_local_aabb()
    }
}
