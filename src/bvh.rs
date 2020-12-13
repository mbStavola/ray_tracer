use std::cmp::Ordering;

use rand::Rng;

use crate::{
    aabb::AABB,
    hittable::{Hit, Hittable, Shape},
    ray::Ray,
    util::RandomDouble,
};

#[derive(Debug)]
pub struct BoundingVolumeHierarchy {
    shapes: Vec<Shape>,
    root: BVHMember,
}

impl<'a> BoundingVolumeHierarchy {
    pub fn new<T: Rng>(
        rng: &mut T,
        shapes: Vec<Shape>,
        time_initial: f64,
        time_final: f64,
    ) -> BoundingVolumeHierarchy {
        let mut shapes = shapes;
        let root = BVHMember::new(rng, &mut shapes, time_initial, time_final);

        BoundingVolumeHierarchy { shapes, root }
    }

    pub fn shapes(&self) -> &Vec<Shape> {
        &self.shapes
    }

    pub fn level_order(&self) -> Vec<&BVHMember> {
        self.root.level_order()
    }
}

impl<'a, T: Rng> Hittable<'a, T> for BoundingVolumeHierarchy {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        self.root.indexed_hit(&self.shapes, ray, t_min, t_max)
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AABB> {
        self.root.bounds().cloned()
    }
}

#[derive(Debug)]
pub enum BVHMember {
    Node {
        bounds: AABB,
        left: Option<Box<BVHMember>>,
        right: Option<Box<BVHMember>>,
    },
    Leaf(usize),
}

impl BVHMember {
    pub fn new<T: Rng>(
        rng: &mut T,
        shapes: &mut [Shape],
        time_initial: f64,
        time_final: f64,
    ) -> BVHMember {
        BVHMember::build_tree(rng, shapes, 0, shapes.len(), time_initial, time_final)
    }

    pub fn bounds(&self) -> Option<&AABB> {
        match self {
            BVHMember::Node { bounds, .. } => Some(bounds),
            BVHMember::Leaf(_) => None,
        }
    }

    pub fn index(&self) -> Option<usize> {
        match self {
            BVHMember::Node { .. } => None,
            BVHMember::Leaf(index) => Some(*index),
        }
    }

    pub fn left(&self) -> Option<&BVHMember> {
        match self {
            BVHMember::Node { left, .. } => left.as_deref(),
            BVHMember::Leaf(_) => None,
        }
    }

    pub fn right(&self) -> Option<&BVHMember> {
        match self {
            BVHMember::Node { right, .. } => right.as_deref(),
            BVHMember::Leaf(_) => None,
        }
    }

    fn build_tree<T: Rng>(
        rng: &mut T,
        shapes: &mut [Shape],
        offset: usize,
        n: usize,
        time_initial: f64,
        time_final: f64,
    ) -> BVHMember {
        let axis = (3.0 * rng.random_double()) as usize;
        shapes.sort_by(|a, b| {
            let a: &dyn Hittable<'_, T> = a;
            let b: &dyn Hittable<'_, T> = b;

            let left = a
                .bounding_box(time_initial, time_final)
                .expect("should exist");
            let right = b
                .bounding_box(time_initial, time_final)
                .expect("should exist");

            let left_axis = left.min()[axis];
            let right_axis = right.min()[axis];

            if left_axis - right_axis < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let bounding_box = |left: &dyn Hittable<'_, T>, right: &dyn Hittable<'_, T>| {
            let left = left
                .bounding_box(time_initial, time_final)
                .expect("should exist");
            let right = right
                .bounding_box(time_initial, time_final)
                .expect("should exist");

            left.surrounding_box(&right)
        };

        if n == 1 {
            BVHMember::Leaf(offset)
        } else if n == 2 {
            let left_hittable = &shapes[0];
            let right_hittable = &shapes[1];

            let bounds = bounding_box(left_hittable, right_hittable);

            let left_leaf = BVHMember::Leaf(offset);
            let right_leaf = BVHMember::Leaf(offset + 1);

            BVHMember::Node {
                left: Some(Box::new(right_leaf)),
                right: Some(Box::new(left_leaf)),
                bounds,
            }
        } else {
            let halving_point = n / 2;
            let left_node = BVHMember::build_tree(
                rng,
                &mut shapes[..halving_point],
                offset,
                halving_point,
                time_initial,
                time_final,
            );
            let right_node = BVHMember::build_tree(
                rng,
                &mut shapes[halving_point..],
                offset + halving_point,
                n - halving_point,
                time_initial,
                time_final,
            );

            let shapes = &shapes as &dyn Hittable<'_, T>;
            let bounds = shapes.bounding_box(time_initial, time_final).expect("");

            BVHMember::Node {
                left: Some(Box::new(left_node)),
                right: Some(Box::new(right_node)),
                bounds,
            }
        }
    }

    fn indexed_hit<'a, T: Rng>(
        &'a self,
        shapes: &'a [Shape],
        ray: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<Hit<'a, T>> {
        let (bounds, left, right) = match self {
            BVHMember::Leaf(index) => {
                let shape = &shapes[*index];
                return shape.hit(ray, t_min, t_max);
            }
            BVHMember::Node {
                bounds,
                left,
                right,
            } => (bounds, left, right),
        };

        if !bounds.hit(ray, t_min, t_max) {
            return None;
        }

        let left_hit = left
            .as_ref()
            .and_then(|it| it.indexed_hit(shapes, ray, t_min, t_max));
        let right_hit = right
            .as_ref()
            .and_then(|it| it.indexed_hit(shapes, ray, t_min, t_max));

        match (left_hit, right_hit) {
            (Some(left_hit), Some(right_hit)) => {
                if left_hit.t() < right_hit.t() {
                    Some(left_hit)
                } else {
                    Some(right_hit)
                }
            }
            (Some(left_hit), None) => Some(left_hit),
            (None, Some(right_hit)) => Some(right_hit),
            (None, None) => None,
        }
    }

    pub fn level_order(&self) -> Vec<&BVHMember> {
        let mut level = Vec::with_capacity(3);
        level.push(self);

        if let BVHMember::Node { left, right, .. } = self {
            if let Some(node) = left {
                level.push(node);
            }

            if let Some(node) = right {
                level.push(node);
            }

            if let Some(nodes) = left.as_ref().map(|it| it.level_order()) {
                level.extend(&nodes[1..]);
            }

            if let Some(nodes) = right.as_ref().map(|it| it.level_order()) {
                level.extend(&nodes[1..]);
            }
        }

        level
    }
}
