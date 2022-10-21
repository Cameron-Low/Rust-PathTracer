use fastrand::Rng;
use crate::math::Ray;
use crate::aabb::{Aabb, surrounding_box, box_compare};
use crate::objects::{Hittable, Elem, Intersection};

pub enum BvhTree {
    Leaf{obj: Elem},
    Node{left: Box<BvhTree>, right: Box<BvhTree>, bbox: Aabb},
}

impl Hittable for BvhTree {
    fn intersect<'a>(&'a self, r: &mut Ray, i: &mut Intersection<'a>) {
        match self {
            BvhTree::Leaf { obj } => obj.intersect(r, i),
            BvhTree::Node { left, right, bbox } => {
                if bbox.hit(r, i.min, i.max) {
                    left.intersect(r, i);
                    right.intersect(r, i);
                }
            }
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Aabb {
        match self {
            BvhTree::Leaf { obj } => obj.bounding_box(time0, time1),
            BvhTree::Node { bbox, .. } => *bbox,
        }
    }
}

impl BvhTree {
    pub fn new(mut objs: Vec<Elem>, time0: f64, time1: f64) -> BvhTree {
        if objs.len() == 1 {
            return BvhTree::Leaf{ obj: objs.pop().unwrap() };
        }

        let rng = Rng::new();
        let axis: usize = rng.u8(0..=2).into();
        objs.sort_by(|a, b| box_compare(&a.bounding_box(time0, time1), &b.bounding_box(time0, time1), axis));

        let mid = objs.len() / 2;
        let right = Box::new(Self::new(objs.drain(mid..).collect(), time0, time1));
        let left = Box::new(Self::new(objs, time0, time1));

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box (time0, time1); 
        let bbox = surrounding_box(&box_left, &box_right);
        
        BvhTree::Node { left, right, bbox}
    }

    pub fn size(&self) -> usize {
        match self {
            BvhTree::Leaf { obj: _ } => 1,
            BvhTree::Node { left, right, bbox: _ } => left.size() + right.size(),
        }
    }
}

pub struct LinearBvh {
    lbvh: Vec<Result<(Aabb, usize), Elem>>,
}

impl LinearBvh {
    pub fn new(bvh: BvhTree) -> Self {
        LinearBvh { lbvh: Self::flatten_bvh(bvh) }
    }

    fn flatten_bvh(bvh: BvhTree) -> Vec<Result<(Aabb, usize), Elem>> {
        match bvh {
            BvhTree::Leaf { obj } => vec![Err(obj)],
            BvhTree::Node { left, right, bbox } => {
                let mut left = Self::flatten_bvh(*left);
                let mut right = Self::flatten_bvh(*right);
                let mut flat_bvh = vec![Ok((bbox, left.len() + right.len()))]; 

                flat_bvh.append(&mut left);
                flat_bvh.append(&mut right);

                flat_bvh
            },
        }
    }
}

impl Hittable for LinearBvh {
    fn intersect<'a>(&'a self, r: &mut Ray, i: &mut Intersection<'a>) {
        let mut ix = 0;
        while ix < self.lbvh.len() {
            let val = &self.lbvh[ix];
            match val {
                Ok((bbox, offset)) => {
                    if !bbox.hit(r, i.min, i.max) {
                        ix += *offset;
                    }
                },
                Err(obj) => obj.intersect(r, i),
            }
            ix += 1;
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Aabb {
        match &self.lbvh[0] {
            Ok((bbox, _)) => *bbox,
            Err(obj) => obj.bounding_box(time0, time1),
        }
    }
}

