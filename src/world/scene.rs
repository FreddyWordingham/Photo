//! Scene collection structure.

use crate::{
    builder::BvhBuilder,
    geometry::{Bvh, Ray},
    render::Contact,
    world::{Entity, Light},
};

/// Collection of entities and lights.
pub struct Scene<'a> {
    /// Lights in the scene.
    lights: Vec<Light>,
    /// Entities in the scene.
    entities: Vec<Entity<'a>>,
    /// Bounding Volume Hierarchy.
    bvh: Bvh,
}

impl<'a> Scene<'a> {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new(
        lights: Vec<Light>,
        entities: Vec<Entity<'a>>,
        bvh_max_children: usize,
        bvh_max_depth: usize,
    ) -> Self {
        debug_assert!(
            bvh_max_children >= 2,
            "Mesh BVH max children must be greater than 2!"
        );
        debug_assert!(bvh_max_depth > 0, "Mesh BVH max depth must be positive!");

        let bvh = BvhBuilder::new().build(&entities, bvh_max_children, bvh_max_depth);

        Self {
            lights,
            entities,
            bvh,
        }
    }

    /// Test for an intersection distance with a [`Ray`].
    #[must_use]
    #[inline]
    pub fn ray_intersect(&self, ray: &Ray) -> bool {
        self.bvh
            .ray_intersections(ray, &self.entities)
            .into_iter()
            .any(|(n, _)| self.entities[n].ray_intersect(ray))
    }

    /// Test for an intersection [`Ray`],
    /// return the distance to the intersection point, if one exists.
    ///
    /// # Panics
    ///
    /// If the comparison between intersection distances fails.
    #[must_use]
    #[inline]
    #[allow(clippy::unwrap_used)]
    pub fn ray_intersect_distance(&self, ray: &Ray) -> Option<f64> {
        self.bvh
            .ray_intersections(ray, &self.entities)
            .into_iter()
            .filter_map(|(n, _)| self.entities[n].ray_intersect_distance(ray))
            .min_by(|distance_a, distance_b| distance_a.partial_cmp(distance_b).unwrap())
    }

    /// Test for an intersection with a [`Ray`],
    /// return the properties of the [`Contact`] point, if one exists.
    ///
    /// # Panics
    ///
    /// If the comparison between intersection distances fails.
    #[must_use]
    #[inline]
    #[allow(clippy::unwrap_used)]
    pub fn ray_intersect_contact(&self, ray: &Ray) -> Option<Contact> {
        self.bvh
            .ray_intersections(ray, &self.entities)
            .into_iter()
            .filter_map(|(n, _)| self.entities[n].ray_intersect_hit(ray))
            .min_by(|contact_a, contact_b| {
                contact_a.distance.partial_cmp(&contact_b.distance).unwrap()
            })
    }
}
