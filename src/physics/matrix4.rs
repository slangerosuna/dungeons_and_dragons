use crate::physics::*;

pub struct Matrix4(Box<[f32; 16]>);
pub struct Matrix3(Box<[f32; 9]>);
pub struct Matrix2(Box<[f32; 4]>);

impl Matrix3 {
    pub fn new() -> Matrix3 {
        Matrix3(Box::new([0.0; 9]))
    }

    pub fn determinant(&self) -> f32 {
        let mut det = 0.0;
        for i in 0..3 {
            det += self.get(i, 0) 
                * self.cofactor(i as i32, 0);
        }
        det
    }

    fn cofactor(&self, i: i32, j: i32) -> f32 {
        let mut sign = 1.0;
        if (i + j) % 2 == 1 {
            sign = -1.0;
        }
        sign * self.minor(i, j)
    }

    fn minor(&self, i: i32, j: i32) -> f32 {
        let mut m = Matrix2::new();
        let mut mi = 0;
        let mut mj = 0;

        let mut sub: u8 = 0;
        for ii in 0..3 {
            if ii == i {
                sub = 1;
                continue;
            }
            mj = 0;
            let mut sub2: u8 = 0;
            for jj in 0..3 {
                if jj == j {
                    sub2 = 1;
                    continue;
                }
                m.set(mi, mj, 
                      self.get(ii as u8 - sub, 
                               jj as u8 - sub2) 
                        as i32);
                mj += 1;
            }
            mi += 1;
        }
        m.determinant()
    }

    pub fn set(&mut self, i: u8, j: u8, val: i32) {
        self.0[(i * 3 + j) as usize] = val as f32;
    }

    pub fn get(&self, i: u8, j: u8) -> f32 {
        self.0[(i * 3 + j) as usize]
    }
}

impl Matrix2 {
    pub fn new() -> Matrix2 {
        Matrix2(Box::new([0.0; 4]))
    }
    
    pub fn determinant(&self) -> f32 {
        self.get(0, 0) * self.get(1, 1) 
        - self.get(0, 1) * self.get(1, 0)
    }

    pub fn set(&mut self, i: u8, j: u8, val: i32) {
        self.0[(i * 2 + j) as usize] = val as f32;
    }

    pub fn get(&self, i: u8, j: u8) -> f32 {
        self.0[(i * 2 + j) as usize]
    }
}

impl Matrix4 {
    pub fn inverse(&self) -> Self {
        let mut m = Self::new();
        let det = self.determinant();
        for i in 0..4 {
            for j in 0..4 {
                m.set(i, j, 
                    self.cofactor(j as i32, i as i32) 
                        / det);
            }
        }
        m
    }
    
    pub fn determinant(&self) -> f32 {
        let mut det = 0.0;
        for i in 0..4 {
            det += self.get(i, 0) 
                * self.cofactor(i as i32, 0);
        }
        det
    }

    fn cofactor(&self, i: i32, j: i32) -> f32 {
        let mut sign = 1.0;
        if (i + j) % 2 == 1 {
            sign = -1.0;
        }
        sign * self.minor(i, j)
    }

    fn minor(&self, i: i32, j: i32) -> f32 {
        let mut m = Matrix3::new();
        let mut mi = 0;
        let mut mj = 0;

        let mut sub: u8 = 0;
        for ii in 0..4 {
            if ii == i {
                sub = 1;
                continue;
            }
            mj = 0;
            let mut sub2: u8 = 0;
            for jj in 0..4 {
                if jj == j {
                    sub2 = 1;
                    continue;
                }
                m.set(mi, mj, 
                    self.get(ii as u8 - sub, 
                             jj as u8 - sub2) 
                        as i32);
                mj += 1;
            }
            mi += 1;
        }
        m.determinant()
    }

    pub fn set(&mut self, x: u8, y: u8, value: f32) {
        self.0[(x + y * 4) as usize] = value;
    }
    pub fn get(&self, x: u8, y: u8) -> f32 {
        self.0[(x + y * 4) as usize]
    }
    pub fn new() -> Self {
        Matrix4(Box::new([0.0; 16]))
    }
    pub fn from_slice(values: &[f32; 16]) -> Self {
        Matrix4(Box::new(*values))
    }
    pub fn identity() -> Self {
        let mut m = Self::new();
        _ = (0..4).map(|i| m.set(i, i, 1.0));
        m
    }
    pub fn multiply(&self, other: &Self) -> Self {
        let mut m = Self::new();
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.get(i, k) 
                        * other.get(k, j);
                }
                m.set(i, j, sum);
            }
        }
        m
    }
    //Angle in radians
    pub fn rotation_matrix(euler_angles: Vector3) -> Self {
        let mut m = Self::identity();
        
        m = m.multiply(
                &Matrix4::axis_angle(
                    Vector3::new(1.0,0.0,0.0), 
                euler_angles.x)
            );
        m = m.multiply(
                &Matrix4::axis_angle(
                    Vector3::new(0.0,1.0,0.0), 
                euler_angles.y)
            );
        m = m.multiply(
                &Matrix4::axis_angle(
                    Vector3::new(0.0,0.0,1.0), 
                euler_angles.z)
            );

        m
    }

    //Angle in radians
    pub fn axis_angle
        (axis: Vector3, angle: f32) -> Self {
            let mut m = Self::new();

            let s = f32::sin(angle);
            let c = f32::cos(angle);
            let _1subc = 1.0 - c;

            m.set(0, 0, 
                  _1subc * axis.x * axis.x + c);
            m.set(1, 0, 
                  _1subc * axis.x * axis.y 
                  + s * axis.z);
            m.set(2, 0, 
                  _1subc * axis.x * axis.z 
                  - s * axis.y);

            m.set(0, 1, 
                  _1subc * axis.x * axis.y 
                  - s * axis.z);
            m.set(1, 1, 
                  _1subc * axis.y * axis.y + c);
            m.set(2, 1, 
                  _1subc * axis.y * axis.z 
                  + s * axis.x);

            m.set(0, 2, 
                  _1subc * axis.x * axis.z 
                  + s * axis.y);
            m.set(1, 2, 
                  _1subc * axis.y * axis.z 
                  - s * axis.x);
            m.set(2, 2, 
                  _1subc * axis.z * axis.z + c);
            
            m.set(3, 3, 1.0);

            m
    }

    pub fn multiply_vector
        (&self, other: &Vector3) -> Vector3 {
            let m = self
                    .multiply(&Self::from_vector(other));

            m.to_vector()
    }

    pub fn from_vector(v: &Vector3) -> Self {
        let mut m = Self::new();
        
        m.set(3, 0, v.x);
        m.set(3, 1, v.y);
        m.set(3, 2, v.z);
        
        m
    }

    pub fn to_vector(&self) -> Vector3 {
        Vector3::new(self.get(3, 0), 
                     self.get(3, 1), 
                     self.get(3, 2))
    }
}
