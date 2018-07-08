
#[derive(Debug, Clone, Copy)]
pub struct V2(pub f32, pub f32);

impl V2 {    
    pub fn scale(&self, a: f32) -> V2 {
        let V2(x, y) = self;
        V2(a * x, a * y)
    }

    pub fn scale_2d(&self, sx: f32, sy:f32) -> V2 {
        let V2(x,y) = self;
        V2(sx * x, sy * y)
    }

    pub fn mod_euc(&self, xmod: f32, ymod: f32) -> V2 {
        let V2(x,y) = self;
        V2(x.mod_euc(xmod), y.mod_euc(ymod))
    }

    pub fn add(&self, w: V2) -> V2 {
        let V2(vx, vy) = self;
        let V2(wx, wy) = w;
        V2(vx + wx, vy + wy)
    }

    pub fn rotate(&self, theta: f32) -> V2 {
        let (sin_th, cos_th) = theta.sin_cos();
        let V2(vx, vy) = self;
        V2(
            vx * cos_th  + vy * sin_th,
            vx * -sin_th + vy * cos_th
        )
    }
}
