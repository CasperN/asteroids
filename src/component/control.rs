use component::{Momentum, Shooting};
use user_interface::UserInput;
use vector_2d::V2;

pub struct Control {
    pub torque: f32,
    pub thrust: f32,
}

impl Control {
    pub fn update(&self, user_input: &UserInput, momentum: &mut Momentum, shooting: &mut Shooting) {
        let torque = -user_input.lr as f32 * self.torque;
        let force = V2(0.0, self.thrust * user_input.ud as f32).rotate(momentum.theta);

        let dt = user_input.elapsed_time();
        momentum.impart(force, torque, dt);

        shooting.is_firing = user_input.shoot;
    }
}
