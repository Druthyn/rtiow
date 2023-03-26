use std::cmp::Ordering;

use rand::{thread_rng, Rng};

use crate::{shapes::{Hit, HitRecord, HittableList}, ray::Ray};

use self::aabb::Aabb;

pub mod aabb;

pub struct BvhNode {
    left: Box<dyn Hit>,
    right: Box<dyn Hit>,
    r#box: Aabb,
}

impl BvhNode {

    // Aim: Build up a BVH from an input


    // Input:  
    // Some sort of "list" of Box<dyn Hit>. SLice = reference to part of a string
    // Would a slice work? We need to move elements around (sort the list via the comparator) and split the list in two, recursively calling 
    // on each sublist. Do borrowing rules permit this?

    // Output: A BVH Node that is the root of the BVH tree built out of the input.


    // Need it be recursive? If this poses borrow check problems, investigate other methods, Currying?
    
    pub fn new(src_objects: &mut [Box<dyn Hit>], start: usize, end: usize, time0: f64, time1: f64) -> BvhNode {
        

        let mut rng = thread_rng();
        let axis: u32 = rng.gen_range(0..3);
        let comparator = |a,b| Self::box_compare(a, b, axis);
        
        let object_span = end-start;
        let left;
        let right;

        match object_span {
            1 => {
                left = src_objects[start];
                right = src_objects[start];
            },
            _ => {
                src_objects.sort_by(|a, b| if comparator(*a, *b) {Ordering::Less} else {Ordering::Greater});

                let mid = start + object_span/(2 as usize);

                left = Box::new(BvhNode::new(src_objects, start, mid, time0, time1));
                right = Box::new(BvhNode::new(src_objects, mid, end, time0, time1));
            },
        }

        let box_left = left.bounding_box(0.0, 0.0);
        let box_right = right.bounding_box(0.0, 0.0);

        if box_left.is_none() || box_right.is_none() {
            eprint!("No bounding box in bvh_node constructor.")
        }

        

        let r#box = Aabb::surrounding_box(box_left.unwrap(), box_right.unwrap());

        BvhNode { left, right, r#box}
    }


    fn box_compare(a: Box<dyn Hit>, b: Box<dyn Hit>, axis: u32) -> bool {
        let box_a = a.bounding_box(0.0, 0.0);
        let box_b = b.bounding_box(0.0, 0.0);

        if box_a.is_none() || box_b.is_none() {
            eprint!("No bounding box in bvh_node constructor.")
        }

        box_a.unwrap().min()[axis] < box_b.unwrap().min()[axis]

    }
}

impl Hit for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.r#box.hit(r, t_min, t_max)?;
        
        
        let hit_left = self.left.hit(r, t_min, t_max);

        let t = match hit_left.as_ref() {
            Some(rec) => rec.t(),
            None => t_max,
        };
            
        let hit_right = self.left.hit(r, t_min, t);
        

        if hit_right.is_some() {
            return hit_right;
        }
        hit_left
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.r#box)
    }
}