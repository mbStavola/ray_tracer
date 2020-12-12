use rand::Rng;

use crate::{
    aabb::AABB,
    material::{Material, Scatterable},
    ray::Ray,
    vec3::Vec3,
};

pub trait Hittable<'a, T: Rng>: Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>>;
    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB>;
}

pub struct Hit<'a, T: Rng> {
    t: f64,
    p: Vec3,
    normal: Vec3,
    material: &'a dyn Scatterable<T>,
}

impl<T: Rng> Hit<'_, T> {
    pub fn new(t: f64, p: Vec3, normal: Vec3, material: &'_ dyn Scatterable<T>) -> Hit<'_, T> {
        Hit {
            t,
            p,
            normal,
            material,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> &Vec3 {
        &self.p
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn material(&self) -> &'_ dyn Scatterable<T> {
        self.material
    }
}

#[derive(Debug)]
pub struct Sphere {
    center_initial: Vec3,
    center_final: Vec3,
    radius: f64,
    material: Material,
    time_start: f64,
    time_end: f64,
}

impl Sphere {
    pub fn new(
        center_initial: Vec3,
        center_final: Vec3,
        radius: f64,
        material: Material,
        time_start: f64,
        time_end: f64,
    ) -> Sphere {
        Sphere {
            center_initial,
            center_final,
            radius,
            material,
            time_start,
            time_end,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        let elapsed_time = (time - self.time_start) as f64;
        let movement_time = (self.time_end - self.time_start) as f64;

        let distance = &self.center_final - &self.center_initial;

        &self.center_initial + ((elapsed_time / movement_time) * &distance)
    }
}

impl<'a, T: Rng> Hittable<'a, T> for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        let oc = ray.origin() - &self.center(ray.time());

        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let discriminant = discriminant.sqrt();

        let f = (-b - discriminant) / a;
        let g = (-b + discriminant) / a;

        let t = if f < t_max && f > t_min {
            f
        } else if g < t_max && g > t_min {
            g
        } else {
            return None;
        };

        let p = ray.point_at(t);
        let normal = (&p - &self.center(ray.time())) / self.radius;
        let material = &self.material;

        let hit = Hit::new(t, p, normal, material);

        Some(hit)
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        let f = |center: &Vec3| {
            let min = center - Vec3::new(self.radius, self.radius, self.radius);
            let max = center + Vec3::new(self.radius, self.radius, self.radius);

            AABB::new(min, max)
        };

        if self.center_initial == self.center_final {
            let aabb = f(&self.center_initial);
            return Some(aabb);
        }

        let initial_box = f(&self.center_initial);
        let final_box = f(&self.center_final);

        let surrounding_box = initial_box.surrounding_box(&final_box);

        Some(surrounding_box)
    }
}

#[derive(Debug)]
pub enum Shape {
    Sphere(Sphere),
}

impl Shape {
    pub fn sphere(x: f64, y: f64, z: f64, radius: f64, material: Material) -> Shape {
        let center = Vec3::new(x, y, z);
        let sphere: Sphere = Sphere::new(center.clone(), center, radius, material, 0.0, 1.0);
        Shape::Sphere(sphere)
    }

    pub fn moving_sphere(
        x0: f64,
        y0: f64,
        z0: f64,
        x1: f64,
        y1: f64,
        z1: f64,
        radius: f64,
        material: Material,
        time_start: f64,
        time_end: f64,
    ) -> Shape {
        let center_initial = Vec3::new(x0, y0, z0);
        let center_final = Vec3::new(x1, y1, z1);
        let sphere: Sphere = Sphere::new(
            center_initial,
            center_final,
            radius,
            material,
            time_start,
            time_end,
        );
        Shape::Sphere(sphere)
    }
}

impl<'a, T: Rng> Hittable<'a, T> for Shape {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        match self {
            Shape::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        match self {
            Shape::Sphere(sphere) => {
                let sphere: &dyn Hittable<'a, T> = sphere;
                sphere.bounding_box(time_start, time_end)
            }
        }
    }
}

impl<'a, T: Rng> Hittable<'a, T> for Vec<Shape> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        let shapes = &self[..];
        hit(shapes, ray, t_min, t_max)
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        bounding_box::<T>(self, time_start, time_end)
    }
}

impl<'a, T: Rng> Hittable<'a, T> for &'a mut [Shape] {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        hit(self, ray, t_min, t_max)
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        bounding_box::<T>(self, time_start, time_end)
    }
}

impl<'a, T: Rng> Hittable<'a, T> for &'a [Shape] {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        hit(self, ray, t_min, t_max)
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        bounding_box::<T>(self, time_start, time_end)
    }
}

fn hit<'a, T: Rng>(shapes: &'a [Shape], ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'a, T>> {
    let mut min_distance = t_max;
    let mut nearest_hit = None;

    for hittable in shapes.iter() {
        if let Some(hit) = hittable.hit(ray, t_min, min_distance) {
            min_distance = hit.t();
            nearest_hit = Some(hit);
        }
    }

    nearest_hit
}

fn bounding_box<'a, T: Rng>(shapes: &[Shape], time_start: f64, time_end: f64) -> Option<AABB> {
    let mut aabb = shapes.first().and_then(|it| {
        let it: &dyn Hittable<'a, T> = it;
        it.bounding_box(time_start, time_end)
    })?;

    for shape in shapes.iter().skip(1) {
        let shape: &dyn Hittable<'a, T> = shape;

        if let Some(new_aabb) = shape.bounding_box(time_start, time_end) {
            aabb = aabb.surrounding_box(&new_aabb);
            continue;
        }

        return None;
    }

    Some(aabb)
}
