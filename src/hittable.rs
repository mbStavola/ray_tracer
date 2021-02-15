use rand::Rng;

use crate::{
    aabb::AABB,
    material::{Material, Scatterable},
    ray::Ray,
    vec3::Vec3,
};
use std::fmt::Debug;

pub trait Hittable<'a, T: Rng>: Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>>;
    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB>;
}

pub struct Hit<'a, T: Rng> {
    t: f64,
    u: f64,
    v: f64,
    p: Vec3,
    front_facing: bool,
    normal: Vec3,
    material: &'a dyn Scatterable<T>,
}

impl<T: Rng> Hit<'_, T> {
    pub fn new<'a>(
        t: f64,
        u: f64,
        v: f64,
        p: Vec3,
        ray: &Ray,
        outward_normal: Vec3,
        material: &'a dyn Scatterable<T>,
    ) -> Hit<'a, T> {
        let front_facing = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_facing {
            outward_normal
        } else {
            -outward_normal
        };

        Hit {
            t,
            u,
            v,
            p,
            front_facing,
            normal,
            material,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
    }

    pub fn p(&self) -> &Vec3 {
        &self.p
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn is_front_facing(&self) -> bool {
        self.front_facing
    }

    pub fn material(&self) -> &'_ dyn Scatterable<T> {
        self.material
    }
}

pub trait Center {
    fn center(&self, time: f64) -> Vec3;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a, T: Rng> Hittable<'a, T> for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        let oc = ray.origin() - &self.center;

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
        let outward_normal = (&p - &self.center) / self.radius;
        let material = &self.material;

        let (u, v) = sphere_uv(&outward_normal);
        let hit = Hit::new(t, u, v, p, ray, outward_normal, material);

        Some(hit)
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AABB> {
        let min = &self.center - Vec3::new(self.radius, self.radius, self.radius);
        let max = &self.center + Vec3::new(self.radius, self.radius, self.radius);

        Some(AABB::new(min, max))
    }
}

impl Center for Sphere {
    fn center(&self, _time: f64) -> Vec3 {
        self.center.clone()
    }
}

#[derive(Debug)]
pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Material,
}

impl XyRect {
    fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Material) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl Center for XyRect {
    fn center(&self, _time: f64) -> Vec3 {
        let x = self.x1 - self.x0 / 2.0;
        let y = self.y1 - self.y0 / 2.0;

        Vec3::new(x, y, self.k)
    }
}

impl<'a, T: Rng> Hittable<'a, T> for XyRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        let t = (self.k - ray.origin().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let y = ray.origin().y() + t * ray.direction().y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let p = ray.point_at(t);

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let hit = Hit::new(t, u, v, p, ray, outward_normal, &self.material);

        Some(hit)
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AABB> {
        let min = Vec3::new(self.x0, self.y0, self.k - 0.0001);
        let max = Vec3::new(self.x1, self.y1, self.k + 0.0001);

        Some(AABB::new(min, max))
    }
}

#[derive(Debug)]
pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}

impl XzRect {
    fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Center for XzRect {
    fn center(&self, _time: f64) -> Vec3 {
        let x = self.x1 - self.x0 / 2.0;
        let z = self.z1 - self.z0 / 2.0;

        Vec3::new(x, self.k, z)
    }
}

impl<'a, T: Rng> Hittable<'a, T> for XzRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        let t = (self.k - ray.origin().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let z = ray.origin().z() + t * ray.direction().z();

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = ray.point_at(t);

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let hit = Hit::new(t, u, v, p, ray, outward_normal, &self.material);

        Some(hit)
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AABB> {
        let min = Vec3::new(self.x0, self.k - 0.0001, self.z0);
        let max = Vec3::new(self.x1, self.k + 0.0001, self.z1);

        Some(AABB::new(min, max))
    }
}

#[derive(Debug)]
pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}

impl YzRect {
    fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Center for YzRect {
    fn center(&self, _time: f64) -> Vec3 {
        let y = self.y1 - self.y0 / 2.0;
        let z = self.z1 - self.z0 / 2.0;

        Vec3::new(self.k, y, z)
    }
}

impl<'a, T: Rng> Hittable<'a, T> for YzRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        let t = (self.k - ray.origin().x()) / ray.direction().x();
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin().y() + t * ray.direction().y();
        let z = ray.origin().z() + t * ray.direction().z();

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = ray.point_at(t);

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let hit = Hit::new(t, u, v, p, ray, outward_normal, &self.material);

        Some(hit)
    }

    fn bounding_box(&self, _time_start: f64, _time_end: f64) -> Option<AABB> {
        let min = Vec3::new(self.k - 0.0001, self.y0, self.z0);
        let max = Vec3::new(self.k + 0.0001, self.y1, self.z1);

        Some(AABB::new(min, max))
    }
}

#[derive(Debug)]
enum Rect {
    Xy(XyRect),
    Xz(XzRect),
    Yz(YzRect),
}

impl Rect {
    pub fn xy(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Material) -> Self {
        let rect = XyRect::new(x0, x1, y0, y1, k, material);
        Self::Xy(rect)
    }

    pub fn xz(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        let rect = XzRect::new(x0, x1, z0, z1, k, material);
        Self::Xz(rect)
    }

    pub fn yz(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        let rect = YzRect::new(y0, y1, z0, z1, k, material);
        Self::Yz(rect)
    }
}

impl<'a, T: Rng> Hittable<'a, T> for Rect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        match self {
            Rect::Xy(rect) => rect.hit(ray, t_min, t_max),
            Rect::Xz(rect) => rect.hit(ray, t_min, t_max),
            Rect::Yz(rect) => rect.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        match self {
            Rect::Xy(rect) => {
                let rect: &dyn Hittable<'a, T> = rect;
                rect.bounding_box(time_start, time_end)
            }
            Rect::Xz(rect) => {
                let rect: &dyn Hittable<'a, T> = rect;
                rect.bounding_box(time_start, time_end)
            }
            Rect::Yz(rect) => {
                let rect: &dyn Hittable<'a, T> = rect;
                rect.bounding_box(time_start, time_end)
            }
        }
    }
}

#[derive(Debug)]
pub struct Cube {
    min: Vec3,
    max: Vec3,
    sides: [Rect; 6],
}

impl Cube {
    fn new(p0: Vec3, p1: Vec3, material: Material) -> Self {
        let sides = [
            Rect::xy(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), material.clone()),
            Rect::xy(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), material.clone()),
            Rect::xz(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), material.clone()),
            Rect::xz(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), material.clone()),
            Rect::yz(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), material.clone()),
            Rect::yz(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), material),
        ];

        Self {
            min: p0,
            max: p1,
            sides,
        }
    }
}

impl Center for Cube {
    fn center(&self, _time: f64) -> Vec3 {
        let x = self.max.x() - self.min.x();
        let y = self.max.y() - self.min.y();
        let z = self.max.z() - self.min.z();

        Vec3::new(x / 2.0, y / 2.0, z / 2.0)
    }
}

impl<'a, T: Rng> Hittable<'a, T> for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        let mut min_distance = t_max;
        let mut nearest_hit = None;

        for hittable in self.sides.iter() {
            if let Some(hit) = hittable.hit(ray, t_min, min_distance) {
                min_distance = hit.t();
                nearest_hit = Some(hit);
            }
        }

        nearest_hit
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        let mut aabb = self.sides.first().and_then(|it| {
            let it: &dyn Hittable<'a, T> = it;
            it.bounding_box(time_start, time_end)
        })?;

        for shape in self.sides.iter().skip(1) {
            let shape: &dyn Hittable<'a, T> = shape;

            if let Some(new_aabb) = shape.bounding_box(time_start, time_end) {
                aabb = aabb.surrounding_box(&new_aabb);
                continue;
            }

            return None;
        }

        Some(aabb)
    }
}

#[derive(Debug)]
pub struct Moving<T: Center + Debug> {
    object: T,
    center_final: Vec3,
    time_start: f64,
    time_end: f64,
}

impl<T: Center + Debug> Moving<T> {
    fn new(object: T, center_final: Vec3, time_start: f64, time_end: f64) -> Self {
        Self {
            object,
            center_final,
            time_start,
            time_end,
        }
    }
}

impl<'a, T: Rng> Hittable<'a, T> for Moving<Sphere> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        let radius = self.object.radius;
        let oc = ray.origin() - &self.center(ray.time());

        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(&oc) - radius * radius;

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
        let outward_normal = (&p - &self.center(ray.time())) / radius;
        let material = &self.object.material;

        let (u, v) = sphere_uv(&outward_normal);
        let hit = Hit::new(t, u, v, p, ray, outward_normal, material);

        Some(hit)
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        let f = |center: &Vec3| {
            let radius = self.object.radius;

            let min = center - Vec3::new(radius, radius, radius);
            let max = center + Vec3::new(radius, radius, radius);

            AABB::new(min, max)
        };

        let center_initial = self.center(time_start);
        let center_final = self.center(time_end);

        if center_initial == center_final {
            let aabb = f(&center_initial);
            return Some(aabb);
        }

        let initial_box = f(&center_initial);
        let final_box = f(&center_final);

        let surrounding_box = initial_box.surrounding_box(&final_box);

        Some(surrounding_box)
    }
}

impl<T: Center + Debug> Center for Moving<T> {
    fn center(&self, time: f64) -> Vec3 {
        // Always assume t=0 for initial center
        let center_initial = &self.object.center(0.0);

        let elapsed_time = (time - self.time_start) as f64;
        let movement_time = (self.time_end - self.time_start) as f64;

        let distance = &self.center_final - center_initial;
        center_initial + ((elapsed_time / movement_time) * &distance)
    }
}

#[derive(Debug)]
pub enum Shape {
    Sphere(Sphere),
    XyRect(XyRect),
    XzRect(XzRect),
    YzRect(YzRect),
    Cube(Cube),
    MovingSphere(Moving<Sphere>),
}

impl Shape {
    pub fn sphere(x: f64, y: f64, z: f64, radius: f64, material: Material) -> Self {
        let center = Vec3::new(x, y, z);
        let sphere: Sphere = Sphere::new(center, radius, material);
        Self::Sphere(sphere)
    }

    pub fn xy_rect(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Material) -> Self {
        let rect = XyRect::new(x0, x1, y0, y1, k, material);
        Self::XyRect(rect)
    }

    pub fn xz_rect(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        let rect = XzRect::new(x0, x1, z0, z1, k, material);
        Self::XzRect(rect)
    }

    pub fn yz_rect(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        let rect = YzRect::new(y0, y1, z0, z1, k, material);
        Self::YzRect(rect)
    }

    pub fn cube(x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64, material: Material) -> Self {
        let p0 = Vec3::new(x0, y0, z0);
        let p1 = Vec3::new(x1, y1, z1);

        let cube = Cube::new(p0, p1, material);
        Self::Cube(cube)
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
    ) -> Self {
        let center_initial = Vec3::new(x0, y0, z0);
        let center_final = Vec3::new(x1, y1, z1);
        let sphere: Sphere = Sphere::new(center_initial, radius, material);

        let sphere = Moving::new(sphere, center_final, time_start, time_end);

        Self::MovingSphere(sphere)
    }
}

impl<'a, T: Rng> Hittable<'a, T> for Shape {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_, T>> {
        match self {
            Shape::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Shape::XyRect(rect) => rect.hit(ray, t_min, t_max),
            Shape::XzRect(rect) => rect.hit(ray, t_min, t_max),
            Shape::YzRect(rect) => rect.hit(ray, t_min, t_max),
            Shape::Cube(cube) => cube.hit(ray, t_min, t_max),
            Shape::MovingSphere(sphere) => sphere.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        match self {
            Shape::Sphere(sphere) => {
                let sphere: &dyn Hittable<'a, T> = sphere;
                sphere.bounding_box(time_start, time_end)
            }
            Shape::XyRect(rect) => {
                let rect: &dyn Hittable<'a, T> = rect;
                rect.bounding_box(time_start, time_end)
            }
            Shape::XzRect(rect) => {
                let rect: &dyn Hittable<'a, T> = rect;
                rect.bounding_box(time_start, time_end)
            }
            Shape::YzRect(rect) => {
                let rect: &dyn Hittable<'a, T> = rect;
                rect.bounding_box(time_start, time_end)
            }
            Shape::Cube(cube) => {
                let cube: &dyn Hittable<'a, T> = cube;
                cube.bounding_box(time_start, time_end)
            }
            Shape::MovingSphere(sphere) => {
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

fn sphere_uv(p: &Vec3) -> (f64, f64) {
    use std::f64::consts::{PI, TAU};

    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + PI;

    let u = phi / TAU;
    let v = theta / PI;

    (u, v)
}
