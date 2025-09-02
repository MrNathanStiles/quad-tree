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
        f.write_str("qtb x: ").unwrap();
        f.write_str(&self.x.to_string()).unwrap();
        f.write_str(", y: ").unwrap();
        f.write_str(&self.y.to_string()).unwrap();
        f.write_str(", w: ").unwrap();
        f.write_str(&self.w.to_string()).unwrap();
        f.write_str(", h: ").unwrap();
        f.write_str(&self.h.to_string())
    }
}

impl QuadTreeBoundsTs {
    pub fn new(x: i64, y: i64, w: i64, h: i64) -> Self {
        Self { x, y, w, h }
    }
    pub fn contains(&self, other: QuadTreeBoundsTs) -> bool {
        self.x <= other.x
            && self.x + self.w >= other.x + other.w
            && self.y <= other.y
            && self.y + self.h >= other.y + other.h
    }
    pub fn intersects(&self, other: QuadTreeBoundsTs) -> bool {
        if self.x >= other.x + other.w {
            return false;
        }
        if other.x >= self.x + self.w {
            return false;
        }
        if self.y >= other.y + other.h {
            return false;
        }
        if other.y >= self.y + self.h {
            return false;
        }
        true
    }
}
