use core::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Range};

#[derive(Clone, Copy)]
pub struct Vec4 {
    e: [i32; 4]
}

pub type Point4 = Vec4;
pub type Color4 = Vec4;

impl Vec4 {
    pub fn new(e0: i32, e1: i32, e2: i32, e3: i32) -> Vec4 {
        Vec4 {
            e: [e0, e1, e2, e3]
        }
    }

    pub fn x(self) -> i32 {
        self[0]
    }

    pub fn y(self) -> i32 {
        self[1]
    }

    pub fn z(self) -> i32 {
        self[2]
    }

    pub fn w(self) -> i32 {
        self[3]
    }
}

impl Index<usize> for Vec4 {
    type Output = i32;

    fn index(&self, index: usize) -> &i32 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        &mut self.e[index]
    }
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2], self[3] + other[3]]
        }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Vec4) -> () {
        *self = Vec4 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2], self[3] + other[3]]
        };
    }
}

impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Vec4 {
        Vec4 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2], self[3] - other[3]]
        }
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Vec4) -> () {
        *self = Vec4 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2], self[3] - other[3]]
        };
    }
}

impl Mul<i32> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: i32) -> Vec4 {
        Vec4 {
            e: [self[0] * other, self[1] * other, self[2] * other, self[3] * other]
        }
    }
}

impl MulAssign<i32> for Vec4 {
    fn mul_assign(&mut self, other: i32) -> () {
        *self = Vec4 {
            e: [self[0] * other, self[1] * other, self[2] * other, self[3] * other]
        };
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        Vec4 {
            e: [self[0] * other[0], self[1] * other[1], self[2] * other[2], self[3] * other[3]]
        }
    }
}

impl MulAssign<Vec4> for Vec4 {
    fn mul_assign(&mut self, other: Vec4) -> () {
        *self = Vec4 {
            e: [self[0] * other[0], self[1] * other[1], self[2] * other[2], self[3] * other[3]]
        };
    }
}

impl Mul<Vec4> for i32 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        Vec4 {
            e: [self * other[0], self * other[1], self * other[2], self * other[3]]
        }
    }
}

impl Div<i32> for Vec4 {
    type Output = Vec4;

    fn div(self, other: i32) -> Vec4 {
        Vec4 {
            e: [self[0] / other, self[1] / other, self[2] / other, self[3] / other]
        }
    }
}

impl DivAssign<i32> for Vec4 {
    fn div_assign(&mut self, other: i32) -> () {
        *self = Vec4 {
            e: [self[0] / other, self[1] / other, self[3] / other, self[3] / other]
        };
    }
}
