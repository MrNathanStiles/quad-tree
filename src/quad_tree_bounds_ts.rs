use std::fmt::Display;

use serde::Deserialize;


#[derive(Copy, Deserialize)]
pub struct QuadTreeBoundsTs {
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

impl Clone for QuadTreeBoundsTs {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }
}

impl Display for QuadTreeBoundsTs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("qtb top: {}, right: {}, bottom: {}, left: {}, width: {}, height: {}", self.top(), self.right(), self.bottom(), self.left(), self.w, self.h))
    }
}

impl QuadTreeBoundsTs {
    pub fn new(x: i64, y: i64, w: i64, h: i64) -> Self {
        Self { x, y, w, h }
    }

    pub fn top(&self) -> i64 {
        self.y + self.h
    }
    pub fn right(&self) -> i64 {
        self.x + self.w
    }
    pub fn bottom(&self) -> i64 {
        self.y
    }
    pub fn left(&self) -> i64 {
        self.x
    }

    pub fn center(&self) -> (f64, f64) {
        (
            self.x as f64 + self.w as f64 * 0.5,
            self.y as f64 + self.h as f64 * 0.5
        )
    }

    pub fn contains(&self, other: QuadTreeBoundsTs) -> bool {
        let result = self.top() >= other.top() &&
        self.right() >= other.right() &&
        self.bottom() <= other.bottom() &&
        self.left() <= other.left();

        //println!(" * * * CONTAIN TEST * * *");
        //println!("{}", self);
        //println!("{}", other);
        
        //let words = if result { "contains"}else{"does nto contain"};
        //println!("{}", words);
        result
    }

    pub fn intersects(&self, other: QuadTreeBoundsTs) -> bool {
        //println!(" * * * INTERSECT TEST * * *");
        //println!("{}", self);
        //println!("{}", other);
        if self.left() >= other.right() {
            //println!("1 does not intersect");
            return false;
        }
        if other.left() >= self.right() {
            //println!("2 does not intersect");
            return false;
        }
        if self.bottom() >= other.top() {
            //println!("3 does not intersect");
            return false;
        }
        if other.bottom() >= self.top() {
            //println!("4 does not intersect");
            return false;
        }
        //println!("intersects");
        true
    }
}
