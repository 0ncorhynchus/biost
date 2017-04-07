use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    /// Returns a three dimensional vector with given coordinates
    ///
    /// # Arguments
    ///
    /// * `x` - A x coordinate
    /// * `y` - A y coordinate
    /// * `z` - A z coordinate
    ///
    /// # Example
    ///
    /// ```
    /// use biost::Vector3d;
    /// let vector = Vector3d::new(1.0, 2.0, 3.0);
    /// assert_eq!(1.0, vector.x);
    /// assert_eq!(2.0, vector.y);
    /// assert_eq!(3.0, vector.z);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3d { x: x, y: y, z: z }
    }

    /// Returns a three dimensional vector at the origin.
    ///
    /// # Example
    ///
    /// ```
    /// use biost::Vector3d;
    /// let zero = Vector3d::zero();
    /// assert_eq!(0.0, zero.x);
    /// assert_eq!(0.0, zero.y);
    /// assert_eq!(0.0, zero.z);
    /// ```
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl ops::Add for Vector3d {
    type Output = Self;
    fn add(self, vector: Self) -> Self::Output {
        Self::new(self.x + vector.x, self.y + vector.y, self.z + vector.z)
    }
}

impl ops::AddAssign for Vector3d {
    fn add_assign(&mut self, vector: Self) {
        self.x += vector.x;
        self.y += vector.y;
        self.z += vector.z;
    }
}

impl ops::Sub for Vector3d {
    type Output = Self;
    fn sub(self, vector: Self) -> Self::Output {
        Self::new(self.x - vector.x, self.y - vector.y, self.z - vector.z)
    }
}

impl ops::SubAssign for Vector3d {
    fn sub_assign(&mut self, vector: Self) {
        self.x -= vector.x;
        self.y -= vector.y;
        self.z -= vector.z;
    }
}

impl ops::Mul<f32> for Vector3d {
    type Output = Self;
    fn mul(self, value: f32) -> Self::Output {
        Self::new(self.x * value, self.y * value, self.z * value)
    }
}

impl ops::MulAssign<f32> for Vector3d {
    fn mul_assign(&mut self, value: f32) {
        self.x *= value;
        self.y *= value;
        self.z *= value;
    }
}

impl ops::Div<f32> for Vector3d {
    type Output = Self;
    fn div(self, value: f32) -> Self::Output {
        Self::new(self.x / value, self.y / value, self.z / value)
    }
}

impl ops::DivAssign<f32> for Vector3d {
    fn div_assign(&mut self, value: f32) {
        self.x /= value;
        self.y /= value;
        self.z /= value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vector = Vector3d::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, vector.x);
        assert_eq!(2.0, vector.y);
        assert_eq!(3.0, vector.z);
    }

    #[test]
    fn test_zero() {
        let zero = Vector3d::zero();
        assert_eq!(0.0, zero.x);
        assert_eq!(0.0, zero.y);
        assert_eq!(0.0, zero.z);
    }

    #[test]
    fn test_add() {
        let vector1 = Vector3d::new(1.0, 2.0, 3.0);
        let vector2 = Vector3d::new(2.0, 4.0, 6.0);
        let ans = vector1 + vector2;
        assert_eq!(3.0, ans.x);
        assert_eq!(6.0, ans.y);
        assert_eq!(9.0, ans.z);

        // vector1 and vector2 should not change
        assert_eq!(1.0, vector1.x);
        assert_eq!(2.0, vector1.y);
        assert_eq!(3.0, vector1.z);
        assert_eq!(2.0, vector2.x);
        assert_eq!(4.0, vector2.y);
        assert_eq!(6.0, vector2.z);
    }

    #[test]
    fn test_add_assign() {
        let mut vector1 = Vector3d::new(1.0, 2.0, 3.0);
        let vector2 = Vector3d::new(2.0, 4.0, 6.0);
        vector1 += vector2;

        assert_eq!(3.0, vector1.x);
        assert_eq!(6.0, vector1.y);
        assert_eq!(9.0, vector1.z);

        // vector2 should not change
        assert_eq!(2.0, vector2.x);
        assert_eq!(4.0, vector2.y);
        assert_eq!(6.0, vector2.z);
    }

    #[test]
    fn test_sub() {
        let vector1 = Vector3d::new(3.0, 6.0, 9.0);
        let vector2 = Vector3d::new(2.0, 4.0, 6.0);
        let ans = vector1 - vector2;
        assert_eq!(1.0, ans.x);
        assert_eq!(2.0, ans.y);
        assert_eq!(3.0, ans.z);

        // vector1 and vector2 should not change
        assert_eq!(3.0, vector1.x);
        assert_eq!(6.0, vector1.y);
        assert_eq!(9.0, vector1.z);
        assert_eq!(2.0, vector2.x);
        assert_eq!(4.0, vector2.y);
        assert_eq!(6.0, vector2.z);
    }

    #[test]
    fn test_sub_assign() {
        let mut vector1 = Vector3d::new(3.0, 6.0, 9.0);
        let vector2 = Vector3d::new(2.0, 4.0, 6.0);
        vector1 -= vector2;

        assert_eq!(1.0, vector1.x);
        assert_eq!(2.0, vector1.y);
        assert_eq!(3.0, vector1.z);

        // vector2 should not change
        assert_eq!(2.0, vector2.x);
        assert_eq!(4.0, vector2.y);
        assert_eq!(6.0, vector2.z);
    }

    #[test]
    fn test_mul() {
        let vector = Vector3d::new(1.0, 2.0, 3.0);
        let ans = vector * 2.0;
        assert_eq!(2.0, ans.x);
        assert_eq!(4.0, ans.y);
        assert_eq!(6.0, ans.z);

        // vector should not change
        assert_eq!(1.0, vector.x);
        assert_eq!(2.0, vector.y);
        assert_eq!(3.0, vector.z);
    }

    #[test]
    fn test_mul_assign() {
        let mut vector = Vector3d::new(1.0, 2.0, 3.0);
        vector *= 2.0;
        assert_eq!(2.0, vector.x);
        assert_eq!(4.0, vector.y);
        assert_eq!(6.0, vector.z);
    }

    #[test]
    fn test_div() {
        let vector = Vector3d::new(2.0, 4.0, 6.0);
        let ans = vector / 2.0;
        assert_eq!(1.0, ans.x);
        assert_eq!(2.0, ans.y);
        assert_eq!(3.0, ans.z);

        // vector should not change
        assert_eq!(2.0, vector.x);
        assert_eq!(4.0, vector.y);
        assert_eq!(6.0, vector.z);
    }

    #[test]
    fn test_div_assign() {
        let mut vector = Vector3d::new(2.0, 4.0, 6.0);
        vector /= 2.0;
        assert_eq!(1.0, vector.x);
        assert_eq!(2.0, vector.y);
        assert_eq!(3.0, vector.z);
    }
}
