use crate::game::settings;
use nalgebra::Point4;

#[allow(dead_code)]
struct Block {
    pos:        Point4<i32>,
    growth:     Point4<f64>,
    block_type: u128,
    modifiers:  Vec<Block>,
    upkeep:     Vec<Block>,
    state:      Option<bool>,
    active:     Option<bool>,
    real:       Option<bool>,
}
impl Default for Block {
    fn default() -> Self {
        Block::new(
            Point4::new(0, 0, 0, 0),
            Point4::new(0., 1., 1. / settings::<f64>("framerate"), 1.),
            0u128,
            vec![],
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
        block_type: u128,
        modifiers: Vec<Block>,
        upkeep: Vec<Block>,
        state: Option<bool>,
        active: Option<bool>,
        real: Option<bool>,
    ) -> Self {
        Self {
            pos,
            growth,
            block_type,
            modifiers,
            upkeep,
            state,
            active,
            real,
        }
    }
}
