extern crate rand;

use controller::Control;

mod momentum;
pub use self::momentum::*;

mod outlinable;
pub use self::outlinable::Outlinable;

use entities::Projectile;


pub trait Shooter {
    fn maybe_shoot<R: rand::Rng>(&mut self, rng: &mut R) -> Option<Projectile>;
}


pub trait Controllable {
    fn control_update(&mut self, control: &Control);
}
