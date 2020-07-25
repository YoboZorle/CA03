use crate::game::settings;
use nalgebra::Point4;

#[allow(dead_code)]
pub struct Block {
    current: u32,
    step:    u32,

    pos:        Point4<i32>,
    growth:     Point4<f64>,
    block_type: u128,
    modifiers:  Vec<Block>,
    upkeep:     Vec<Block>,
    state:      Option<bool>,
    active:     Option<bool>,
    real:       Option<bool>,
}
pub struct Field {}
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
            current: 0u32,
            step: u32::MAX / (settings::<u32>("framerate") + 1),
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

    pub fn filled(&self) -> f64 { self.current as f64 / u32::MAX as f64 }

    pub fn c(&self) -> f64 { self.filled() * self.growth.y }

    pub fn update(&mut self) {
        if self.current < u32::MAX {
            self.current = self.current.saturating_add(self.step);
        }
    }
}
