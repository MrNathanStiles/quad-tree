use std::fmt::Display;

pub struct QuadTreeBounds {
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

impl Clone for QuadTreeBounds {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone(), w: self.w.clone(), h: self.h.clone() }
    }
}

impl Copy for QuadTreeBounds { }

impl Display for QuadTreeBounds {
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

impl QuadTreeBounds {
    pub fn new(x: i64, y: i64, w: i64, h: i64) -> Self {
        Self { x, y, w, h }
    }
    pub fn contains(&self, other: QuadTreeBounds) -> bool {
        self.x <= other.x
            && self.x + self.w >= other.x + other.w
            && self.y <= other.y
            && self.y + self.h >= other.y + other.h
    }
    pub fn intersects(&self, other: QuadTreeBounds) -> bool {
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
