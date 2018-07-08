use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct V2(pub f32, pub f32);

impl V2 {
    pub fn scale(&self, a: f32) -> V2 {
        V2(self.0 * a, self.1 * a)
    }

    pub fn scale_2d(&self, sx: f32, sy:f32) -> V2 {
        V2(self.0 * sx, self.1 * sy)
    }

    pub fn mod_euc(&self, xmod: f32, ymod: f32) -> V2 {
        V2(self.0.mod_euc(xmod), self.1.mod_euc(ymod))
    }

    pub fn add(&self, other: V2) -> V2 {
        V2(self.0 + other.0, self.1 + other.1)
    }

    pub fn rotate(&self, theta: f32) -> V2 {
        let (sin_th, cos_th) = theta.sin_cos();
        V2(self.0 * cos_th  + self.1 * sin_th, self.0 * -sin_th + self.1 * cos_th)
    }
}


pub fn roots_of_unity(n: usize) -> Vec<V2> {
    (0..n).map( |k| {
        let (y, x) = (2.0 * PI * k as f32 / n as f32).sin_cos();
        V2(x, y)
    }).collect()
}
