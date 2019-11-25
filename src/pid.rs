pub enum ControlMode {
    Position,
    Velocity,
    Current
}

pub trait PIDMotor<R> {
    fn set_p(&mut self, value: f64) -> R;
    fn set_i(&mut self, value: f64) -> R;
    fn set_d(&mut self, value: f64) -> R;
    fn set_control_mode(&mut self, value: f64) -> R;
}