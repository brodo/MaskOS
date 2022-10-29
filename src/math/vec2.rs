use core::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Clone, Copy)]
pub struct Vec2 {
    e: [i32; 2]
}

pub type Point2 = Vec2;
pub type Color2 = Vec2;

impl Vec2 {
    pub fn new(e0: i32, e1: i32) -> Vec2 {
        Vec2 {
            e: [e0, e1]
        }
    }

    pub fn x(self) -> i32 {
        self[0]
    }

    pub fn y(self) -> i32 {
        self[1]
    }
}

impl Index<usize> for Vec2 {
    type Output = i32;

    fn index(&self, index: usize) -> &i32 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        &mut self.e[index]
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            e: [self[0] + other[0], self[1] + other[1]]
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) -> () {
        *self = Vec2 {
            e: [self[0] + other[0], self[1] + other[1]]
        };
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            e: [self[0] - other[0], self[1] - other[1]]
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) -> () {
        *self = Vec2 {
            e: [self[0] - other[0], self[1] - other[1]]
        };
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: i32) -> Vec2 {
        Vec2 {
            e: [self[0] * other, self[1] * other]
        }
    }
}

impl MulAssign<i32> for Vec2 {
    fn mul_assign(&mut self, other: i32) -> () {
        *self = Vec2 {
            e: [self[0] * other, self[1] * other]
        };
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            e: [self[0] * other[0], self[1] * other[1]]
        }
    }
}

impl MulAssign<Vec2> for Vec2 {
    fn mul_assign(&mut self, other: Vec2) -> () {
        *self = Vec2 {
            e: [self[0] * other[0], self[1] * other[1]]
        };
    }
}

impl Mul<Vec2> for i32 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            e: [self * other[0], self * other[1]]
        }
    }
}

impl Div<i32> for Vec2 {
    type Output = Vec2;

    fn div(self, other: i32) -> Vec2 {
        Vec2 {
            e: [self[0] / other, self[1] / other]
        }
    }
}

impl DivAssign<i32> for Vec2 {
    fn div_assign(&mut self, other: i32) -> () {
        *self = Vec2 {
            e: [self[0] / other, self[1] / other]
        };
    }
}
