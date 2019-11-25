pub trait Motor<R> {
    fn set(&mut self, value: f64) -> R;
}