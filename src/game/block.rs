use nalgebra::Point4;

struct Block {
    pos: Point4<i32>,
    growth: Point4<f64>,
    modifiers: Vec<Block>,
    state: Option<bool>,
    active: Option<bool>,
    real: Option<bool>,
}
impl Default for Block {
    fn default() -> Self {
        Block::new(
            Point4::new(0, 0, 0, 0),
            Point4::new(1., 1., 0.01, 1.),
            vec![],
            Some(false),
            Some(false),
            Some(false),
        )
    }
}
impl Block {
    fn new(
        pos: Point4<i32>,
        growth: Point4<f64>,
        modifiers: Vec<Block>,
        state: Option<bool>,
        active: Option<bool>,
        real: Option<bool>,
    ) -> Self {
        Self {
            pos,
            growth,
            modifiers,
            state,
            active,
            real,
        }
    }
}
