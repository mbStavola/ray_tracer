use std::cmp::Ordering;

use rand::Rng;

use crate::{
    aabb::AABB,
    hittable::{Hit, Hittable, Shape},
    ray::Ray,
};

#[derive(Debug)]
pub struct BoundingVolumeHierarchy {
    shapes: Vec<Shape>,
    nodes: Vec<BVHMember>,
}

impl<'a> BoundingVolumeHierarchy {
    pub fn new<T: Rng>(
        rng: &mut T,
        shapes: Vec<Shape>,
        time_initial: f64,
        time_final: f64,
    ) -> BoundingVolumeHierarchy {
        let mut shapes = shapes;
        let nodes = build_tree(rng, &mut shapes, time_initial, time_final);
        BoundingVolumeHierarchy { shapes, nodes }
    }

    pub fn shapes(&self) -> &Vec<Shape> {
        &self.shapes
    }

    fn root(&self) -> &BVHMember {
        &self.nodes[self.nodes.len() - 1]
    }

    fn indexed_hit<T: Rng>(
        &'a self,
        member: &BVHMember,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<Hit<'a, T>> {
        let (bounds, left, right) = match member {
            BVHMember::Leaf(index) => {
                let shape = &self.shapes[*index];
                return shape.hit(ray, t_min, t_max);
            }
            BVHMember::Node {
                bounds,
                left,
                right,
            } => {
                let left = left.and_then(|index| self.nodes.get(index));
                let right = right.and_then(|index| self.nodes.get(index));

                (bounds, left, right)
            }
        };

        if !bounds.hit(ray, t_min, t_max) {
            return None;
        }

        let left_hit = left.and_then(|member| self.indexed_hit(member, ray, t_min, t_max));
        let right_hit = right.and_then(|member| self.indexed_hit(member, ray, t_min, t_max));

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
}

impl<'a, T: Rng> Hittable<'a, T> for BoundingVolumeHierarchy {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        self.indexed_hit(self.root(), ray, t_min, t_max)
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AABB> {
        self.root().bounds().cloned()
    }
}

#[derive(Debug)]
pub enum BVHMember {
    Node {
        bounds: AABB,
        left: Option<usize>,
        right: Option<usize>,
    },
    Leaf(usize),
}

impl BVHMember {
    pub fn bounds(&self) -> Option<&AABB> {
        match self {
            BVHMember::Node { bounds, .. } => Some(bounds),
            BVHMember::Leaf(_) => None,
        }
    }
}

fn build_tree<T: Rng>(
    rng: &mut T,
    shapes: &mut [Shape],
    time_initial: f64,
    time_final: f64,
) -> Vec<BVHMember> {
    let mut nodes = Vec::with_capacity(shapes.len());

    let root = build_tree_internal(
        rng,
        shapes,
        &mut nodes,
        0,
        shapes.len(),
        time_initial,
        time_final,
    );

    // NOTE(Matt): The root is technically at the "end" of the list
    nodes.push(root);

    nodes
}

fn build_tree_internal<T: Rng>(
    rng: &mut T,
    shapes: &mut [Shape],
    nodes: &mut Vec<BVHMember>,
    offset: usize,
    n: usize,
    time_initial: f64,
    time_final: f64,
) -> BVHMember {
    let axis = rng.gen_range(0, 3);
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

        let left_index = nodes.len();
        nodes.push(left_leaf);

        let right_index = nodes.len();
        nodes.push(right_leaf);

        BVHMember::Node {
            left: Some(left_index),
            right: Some(right_index),
            bounds,
        }
    } else {
        let halving_point = n / 2;

        let left = build_tree_internal(
            rng,
            &mut shapes[..halving_point],
            nodes,
            offset,
            halving_point,
            time_initial,
            time_final,
        );
        let left_index = nodes.len();
        nodes.push(left);

        let right = build_tree_internal(
            rng,
            &mut shapes[halving_point..],
            nodes,
            offset + halving_point,
            n - halving_point,
            time_initial,
            time_final,
        );
        let right_index = nodes.len();
        nodes.push(right);

        let shapes = &shapes as &dyn Hittable<'_, T>;
        let bounds = shapes.bounding_box(time_initial, time_final).expect("");

        BVHMember::Node {
            left: Some(left_index),
            right: Some(right_index),
            bounds,
        }
    }
}
