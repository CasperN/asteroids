

pub struct V2(f32, f32);
impl V2 {

    pub fn new(v: (f32, f32)) -> V2 {
        let (x,y) = v;
        return V2(x,y)
    }

    pub fn scale(&self, a: f32) -> V2 {
        let V2(x, y) = self;
        V2(a * x, a * y)
    }

    pub fn add_(&self, wx: f32, wy:f32) -> V2 {
        let V2(vx, vy) = self;
        V2(vx + wx, vy + wy)
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
