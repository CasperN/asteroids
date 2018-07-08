use std::f32::consts::PI;

use vector_2d::V2;
use X_LEN;
use Y_LEN;

#[derive(Debug, Clone, Copy)]
pub struct MomentumC {
    pub pos: V2,
    pub vel: V2,
    pub theta: f32,
    pub omega: f32,
    pub mass: f32, // also moment of inertia
}

pub trait Momentum {
    const SPEED_DECAY: f32;
    const WRAP_AROUND: bool;
    const ROTATION_DECAY: f32;

    fn get_momentum(&self) -> &MomentumC;
    fn get_momentum_mut(&mut self) -> &mut MomentumC;

    fn move_position(&mut self, time: f32) {
        let mc = self.get_momentum_mut();

        mc.pos = mc.pos.add(mc.vel.scale(time));
        if Self::WRAP_AROUND {
            mc.pos = mc.pos.mod_euc(X_LEN, Y_LEN);
        }

        mc.theta += mc.omega * time;
        mc.theta = mc.theta.mod_euc(2.0 * PI);
    }

    fn impart(&mut self, force: V2, torque: f32, time: f32) {
        let mc = self.get_momentum_mut();

        mc.vel = mc.vel.add(force.scale(time / mc.mass))
                       .scale(Self::SPEED_DECAY);

        mc.omega += torque * time / mc.mass;
        mc.omega *= Self::ROTATION_DECAY;
    }
}
