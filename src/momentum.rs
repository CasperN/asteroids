use vector_2d::V2;

pub struct MomentumC {
    pos: V2,
    vel: V2,
    theta: f32,
    omega: f32,
    mass: f32, // also moment of inertia
}

macro_rules! impl_Momentum {
    ($T:ident) => {
        impl Momentum for $T {
            fn get_momentum(&self) -> &MomentumC {
                self.momentum
            }
            fn get_momentum_mut(&mut self) -> &mut MomentumC{
                &mut self.momentum
            }
        }
    }
}


pub trait Momentum {
    const SPEED_DECAY: f32;
    const WRAP_AROUND: bool;
    const ROTATION_DECAY: f32;

    fn get_momentum(&self) -> &MomentumC;
    fn get_momentum_mut(&mut self) -> &mut MomentumC;

    fn move_position(&mut self, time: f32) {
        let mut mc = self.get_momentum_mut();
        mc.pos = mc.pos.add(mc.vel.scale(time));
        mc.theta += mc.omega * time;
    }

    fn impart(&mut self, force: V2, torque: f32, time: f32) {
        let mut mc = self.get_momentum_mut();

        mc.vel = mc.vel.add(force.scale(time / mc.mass))
                       .scale(Self::SPEED_DECAY);

        mc.omega += torque * time / mc.mass;
        mc.omega *= Self::ROTATION_DECAY;
    }
}
