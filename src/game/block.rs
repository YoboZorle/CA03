use crate::game::settings;
use nalgebra::Point4;

#[allow(dead_code)]
pub struct Block {
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
            Point4::new(0., 1., 1. / 1000000. / settings::<f64>("framerate"), 1.),
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
    pub fn new(
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

    pub fn pos(&self) -> Point4<i32> { self.pos }

    pub fn growth(&self) -> Point4<f64> { self.growth }

    pub fn btype(&self) -> u128 { self.block_type }

    pub fn update(&mut self) {
        if self.growth.x < self.growth.y {
            self.growth.x += self.growth.z;
        }
    }
}
