#[derive(Clone, Copy)]
pub struct Matrix4f {
    pub mat: [[f32; 4]; 4],
}

impl Matrix4f {
    pub fn new() -> Matrix4f {
        Matrix4f {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_affine(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        let s: f32 = angle.sin();
        let c: f32 = angle.cos();
        let c_: f32 = 1.0 - c;
        let xx: f32 = x * x;
        let xy: f32 = x * y;
        let xz: f32 = x * z;
        let yy: f32 = y * y;
        let yz: f32 = y * z;
        let zz: f32 = z * z;
        let rm00: f32 = xx * c_ + c;
        let rm01: f32 = xy * c_ + z * s;
        let rm02: f32 = xz * c_ - y * s;
        let rm10: f32 = xy * c_ - z * s;
        let rm11: f32 = yy * c_ + c;
        let rm12: f32 = yz * c_ + x * s;
        let rm20: f32 = xz * c_ + y * s;
        let rm21: f32 = yz * c_ - x * s;
        let rm22: f32 = zz * c_ + c;

        let nm00: f32 = self.mat[0][0] * rm00 + self.mat[1][0] * rm01 + self.mat[2][0] * rm02;
        let nm01: f32 = self.mat[0][1] * rm00 + self.mat[1][1] * rm01 + self.mat[2][1] * rm02;
        let nm02: f32 = self.mat[0][2] * rm00 + self.mat[1][2] * rm01 + self.mat[2][2] * rm02;
        let nm10: f32 = self.mat[0][0] * rm10 + self.mat[1][0] * rm11 + self.mat[2][0] * rm12;
        let nm11: f32 = self.mat[0][1] * rm10 + self.mat[1][1] * rm11 + self.mat[2][1] * rm12;
        let nm12: f32 = self.mat[0][2] * rm10 + self.mat[1][2] * rm11 + self.mat[2][2] * rm12;

        self.mat[2][0] = self.mat[0][0] * rm20 + self.mat[1][0] * rm21 + self.mat[2][0] * rm22;
        self.mat[2][1] = self.mat[0][1] * rm20 + self.mat[1][1] * rm21 + self.mat[2][1] * rm22;
        self.mat[2][2] = self.mat[0][2] * rm20 + self.mat[1][2] * rm21 + self.mat[2][2] * rm22;
        self.mat[2][3] = 0.0;

        self.mat[0][0] = nm00;
        self.mat[0][1] = nm01;
        self.mat[0][2] = nm02;
        self.mat[0][3] = 0.0;
        self.mat[1][0] = nm10;
        self.mat[1][1] = nm11;
        self.mat[1][2] = nm12;
        self.mat[1][3] = 0.0;
    }

    pub fn rotate(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        let sin: f32 = angle.sin();
        let cos: f32 = angle.cos();
        let c_: f32 = 1.0 - cos;
        let xy: f32 = x * y;
        let xz: f32 = x * z;
        let yz: f32 = y * z;
        self.mat[0][0] = cos + x * x * c_;
        self.mat[1][0] = xy * c_ - z * sin;
        self.mat[2][0] = xz * c_ + y * sin;
        self.mat[3][0] = 0.0;
        self.mat[0][1] = xy * c_ + z * sin;
        self.mat[1][1] = cos + y * y * c_;
        self.mat[2][1] = yz * c_ - x * sin;
        self.mat[3][1] = 0.0;
        self.mat[0][2] = xz * c_ - y * sin;
        self.mat[1][2] = yz * c_ + x * sin;
        self.mat[2][2] = cos + z * z * c_;
        self.mat[3][2] = 0.0;
        self.mat[0][3] = 0.0;
        self.mat[1][3] = 0.0;
        self.mat[2][3] = 0.0;
        self.mat[3][3] = 1.0;
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.mat[3][0] =
            self.mat[0][0] * x + self.mat[1][0] * y + self.mat[2][0] * z + self.mat[3][0];
        self.mat[3][1] =
            self.mat[0][1] * x + self.mat[1][1] * y + self.mat[2][1] * z + self.mat[3][1];
        self.mat[3][2] =
            self.mat[0][2] * x + self.mat[1][2] * y + self.mat[2][2] * z + self.mat[3][2];
        self.mat[3][3] =
            self.mat[0][3] * x + self.mat[1][3] * y + self.mat[2][3] * x + self.mat[3][3];
    }

    pub fn as_ptr(&self) -> *const f32 {
        &self.mat[0][0]
    }

    pub fn perspective(fovy: f32, aspect: f32, z_near: f32, z_far: f32) -> Matrix4f {
        let h = (fovy * 0.5).tan();
        let m00 = 1.0 / (h * aspect);
        let m11 = 1.0 / h;
        let m22 = (z_far + z_near) / (z_near - z_far);
        let m32 = (z_far + z_far) * z_near / (z_near - z_far);
        let m23 = -1.0;

        Matrix4f {
            mat: [
                [m00, 0.0, 0.0, 0.0],
                [0.0, m11, 0.0, 0.0],
                [0.0, 0.0, m22, m23],
                [0.0, 0.0, m32, 0.0],
            ],
        }
    }
}
