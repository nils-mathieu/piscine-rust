use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vector<f32> {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Vector<f64> {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl<T: Add<Output = T>> Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: SubAssign> SubAssign for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vector<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vector<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Vector<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Vector<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Eq> Eq for Vector<T> {}

#[cfg(test)]
#[test]
fn test_a() {
    let v = Vector {
        x: String::from("Hello, World!"),
        y: String::from("Hello, Rust!"),
    };

    let w = v.clone();

    assert_eq!(&v, &w);
}

#[cfg(test)]
#[test]
fn test_b() {
    let v = Vector::new("Hello, World!", "Hello, Rust!");
    let a = v;
    let b = v;
    assert_eq!(a, b);
}
