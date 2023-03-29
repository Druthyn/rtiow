use std::cmp::Ordering;

use crate::ray::Ray;
use crate::shapes::Hit;
use crate::shapes::HitRecord;

use self::aabb::Aabb;

pub mod aabb;

// Implementation assistance in Rust was provided from https://github.com/fralken/ray-tracing-the-next-week
//
// Code has been commented to solidify personal understanding and to help me find a viable solution in the future.


enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hit>)
}

pub struct BVH {
    tree: BVHNode,
    bbox: Aabb
}

impl BVH {

    
    pub fn new(mut src_objects: Vec<Box<dyn Hit>>, time0: f64, time1: f64) -> BVH {

        fn box_compare(time0: f64, time1: f64, axis: usize) -> impl FnMut(&Box<dyn Hit>, &Box<dyn Hit>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box(time0, time1);
                let b_bbox = b.bounding_box(time0, time1);
                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.min()[axis] + a.max()[axis];
                    let bc = b.min()[axis] + b.max()[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!["no bounding box in bvh node"]
                }
            }
        }

        
        // Axis selected by choosing the widest range axis of a bounding box
        // (An efficiency change over the original implementation)

        fn axis_range(hitable: &[Box<dyn Hit>], time0: f64, time1: f64, axis: usize) -> f64 {
            let (min, max) = hitable.iter().fold((f64::MAX, f64::MIN), |(bmin, bmax), hit| {
                if let Some(aabb) = hit.bounding_box(time0, time1) {
                    (bmin.min(aabb.min()[axis]), bmax.max(aabb.max()[axis]))
                } else {
                    (bmin, bmax)
                }
            });
            max - min
        }

        let mut axis_ranges: Vec<(usize, f64)> = (0..3)
            .map(|a| (a, axis_range(&src_objects, time0, time1, a)))
            .collect();

        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;


        src_objects.sort_unstable_by(box_compare(time0, time1, axis));

        
        let len = src_objects.len();
        match len {
            0 => panic!("No elements in scene"),
            1 => {
                let leaf = src_objects.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(time0, time1) {
                    BVH {tree: BVHNode::Leaf(leaf), bbox}
                } else {
                    panic!("BVH node without bounding box")
                }
            },
            _ => {
                let right = BVH::new(src_objects.drain(len/2..).collect(), time0, time1);
                let left = BVH::new(src_objects, time0, time1);

                let bbox = Aabb::surrounding_box(&left.bbox, &right.bbox);
                BVH {tree: BVHNode::Branch { left: Box::new(left), right: Box::new(right)}, bbox}
            
            },
        }
    }


    
}

impl Hit for BVH {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.bbox.hit(r, t_min, t_max)?;
        
        match &self.tree {
            BVHNode::Leaf(leaf) => leaf.hit(r, t_min, t_max),
            BVHNode::Branch { left, right } => {
                let hit_left = left.hit(r, t_min, t_max);

                let t = match hit_left.as_ref() {
                    Some(rec) => rec.t(),
                    None => t_max,
                };
                    
                let hit_right = right.hit(r, t_min, t);
                
        
                if hit_right.is_some() {
                    return hit_right;
                }
                hit_left
            }
            
        }
        
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}