use parry3d::{bounding_volume::Aabb, shape::TypedShape};

use super::OxidizedCollider;

/// This is only compiled and available when the "xpbd" feature is enabled.
impl OxidizedCollider for bevy_xpbd_3d::prelude::Collider {
    fn oxidized_into_typed_shape(&self) -> TypedShape {
        self.as_typed_shape()
    }

    fn oxidized_compute_local_aabb(&self) -> Aabb {
        self.compute_local_aabb()
    }
}
