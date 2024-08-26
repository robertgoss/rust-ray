
#[derive(Copy, Clone)]
pub struct Interval {
    pub min : f64,
    pub max : f64
}

impl Interval {
    pub fn universe() -> Interval {
        Interval {
            min : f64::MIN,
            max : f64::MAX
        }
    }

    pub fn empty() -> Interval {
        Interval {
            min : f64::MAX,
            max : f64::MIN
        }
    }

    pub fn unit() -> Interval {
        Interval {
            min : 0.0,
            max : 1.0
        }
    }

    pub fn new(min : f64, max : f64) -> Interval {
        Interval {min, max}
    }

    pub fn from_vals(a : f64, b : f64) -> Interval {
        if a < b {
            Interval::new(a, b)
        } else {
            Interval::new(b, a)
        }
    }

    pub fn about(center : f64, radius : f64) -> Interval {
        Interval { min : center - radius, max : center + radius }
    }

    pub fn contains(&self, x : f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x : f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x : f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn intersect(&self, other : &Interval) -> Option<Interval> {
        let t_min = if self.min > other.min {
            self.min
        } else {
            other.min
        };
        let t_max = if self.max < other.max {
            self.max
        } else {
            other.max
        };
        if t_min < t_max {
            Some(Interval::new(t_min, t_max))
        } else {
            None
        }
    }

    pub fn expand(&mut self, val : f64) {
        if self.min > val {
            self.min = val;
        }
        if self.max < val {
            self.max = val;
        }
    }

    pub fn union(&self, other : &Interval) -> Interval {
        let t_min = if self.min < other.min {
            self.min
        } else {
            other.min
        };
        let t_max = if self.max > other.max {
            self.max
        } else {
            other.max
        };
        Interval::new(t_min, t_max)
    }

    pub fn translate(&self, delta : f64) -> Interval {
        Interval {
            min : self.min + delta,
            max : self.max + delta
        }
    }

    pub fn length(&self) -> f64 {
        if self.max > self.min {
            self.max - self.min
        } else {
            0.0
        }
    }

    pub fn pad(&mut self, min : f64) {
        if self.length() < min {
            let mid = (self.max + self.min) * 0.5;
            self.min = mid - (0.5 * min);
            self.max = mid + (0.5 * min);
        }
    }
}
