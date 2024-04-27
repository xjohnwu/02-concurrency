use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Mul<Output = T> + Add<Output = T> + AddAssign + Send,
{
    if a.len() != b.len() {
        // a.len() => a.data.len() (Deref trait)
        return Err(anyhow!("Vector size mismatch"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}
