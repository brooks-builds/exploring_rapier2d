pub struct Vector2;

impl Vector2 {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(x: f32, y: f32) -> nalgebra::Vector2<f32> {
        let mut vector = nalgebra::Vector2::default();
        vector.x = x;
        vector.y = y;

        vector
    }
}
