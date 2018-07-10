use std::f32::consts::PI;

use vector_2d::V2;
use X_LEN;
use Y_LEN;

#[derive(Debug, Clone, Copy)]
pub struct Inertia {
    pub pos: V2,
    pub vel: V2,
    pub theta: f32,
    pub omega: f32,
    pub mass: f32, // also moment of inertia
}

#[allow(dead_code)]
pub enum EdgeBehaviour {
    Pass,
    PacMan,
    Wall,
}

pub trait Momentum {
    const SPEED_DECAY: f32;
    const EDGE: EdgeBehaviour;
    const ROTATION_DECAY: f32;

    fn get_momentum(&self) -> &Inertia;
    fn get_momentum_mut(&mut self) -> &mut Inertia;

    fn move_position(&mut self, time: f32) {
        let mc = self.get_momentum_mut();

        mc.pos = mc.pos + mc.vel.scale(time);
        match Self::EDGE {
            EdgeBehaviour::PacMan => mc.pos = mc.pos.mod_euc(X_LEN, Y_LEN),
            EdgeBehaviour::Pass => (),
            EdgeBehaviour::Wall => {
                if mc.pos.0 < 0.0 {
                    mc.pos.0 = 0.0;
                    mc.vel.0 = 0.0;
                }
                if mc.pos.1 < 0.0 {
                    mc.pos.1 = 0.0;
                    mc.vel.1 = 0.0;
                }
                if mc.pos.0 > X_LEN {
                    mc.pos.0 = X_LEN;
                    mc.vel.0 = 0.0;
                }
                if mc.pos.1 > Y_LEN {
                    mc.pos.1 = Y_LEN;
                    mc.vel.1 = 0.0;
                }
            }
        }

        mc.theta += mc.omega * time;
        mc.theta = mc.theta.mod_euc(2.0 * PI);
    }

    fn impart(&mut self, force: V2, torque: f32, time: f32) {
        let mc = self.get_momentum_mut();

        mc.vel += force.scale(time / mc.mass);
        mc.vel = mc.vel.scale(Self::SPEED_DECAY.powf(time));

        mc.omega += torque * time / mc.mass;
        mc.omega *= Self::ROTATION_DECAY.powf(time);
    }

    fn in_bounds(&self) -> bool {
        let V2(x, y) = self.get_momentum().pos;
        0.0 <= x && x <= X_LEN && 0.0 <= y && y <= Y_LEN
    }
}
