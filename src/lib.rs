use derive_more::{
    Add, AddAssign, Binary, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,
    BitXorAssign, Display, Div, DivAssign, LowerExp, LowerHex, Mul, MulAssign,
    Neg, Not, Octal, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
    SubAssign, UpperExp, UpperHex,
};
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    Not,
    Shr,
    Shl,
    BitOr,
    BitAnd,
    BitXor,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    ShrAssign,
    ShlAssign,
    BitOrAssign,
    BitAndAssign,
    BitXorAssign,
    Display,
    Binary,
    Octal,
    LowerHex,
    UpperHex,
    LowerExp,
    UpperExp,
)]
#[display(fmt = "[{}, {}, {}]", x, y, z)]
#[binary(fmt = "[{:b}, {:b}, {:b}]", x, y, z)]
#[octal(fmt = "[{:o}, {:o}, {:o}]", x, y, z)]
#[lower_hex(fmt = "[{:x}, {:x}, {:x}]", x, y, z)]
#[upper_hex(fmt = "[{:X}, {:X}, {:X}]", x, y, z)]
#[lower_exp(fmt = "[{:e}, {:e}, {:e}]", x, y, z)]
#[upper_exp(fmt = "[{:E}, {:E}, {:E}]", x, y, z)]
pub struct Vec3<T>
where
    T: Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Index<usize> for Vec3<T>
where
    T: Copy,
{
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("invalid index"),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T>
where
    T: Copy,
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("invalid index"),
        }
    }
}

impl<T> Vec3<T>
where
    T: Copy,
    T: Mul<Output = T> + Add<Output = T>,
{
    pub fn len_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T> Vec3<T>
where
    T: Copy,
    T: Mul<Output = T> + Sub<Output = T>,
{
    pub fn cross(&self, rhs: &Self) -> Vec3<T> {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T> Vec3<T>
where
    T: Copy,
{
    pub fn map<F, U>(&self, mut f: F) -> Vec3<U>
    where
        F: FnMut(T) -> U,
        U: Copy,
    {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}

impl Vec3<f32> {
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn normalized(&self) -> Vec3<f32> {
        *self / self.len()
    }
}

impl Vec3<f64> {
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn normalized(&self) -> Vec3<f64> {
        *self / self.len()
    }
}
