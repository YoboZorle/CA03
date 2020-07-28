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

pub fn step(
    // current board state in the same format  as the mask
    mut data: [u128; 16],
    //to be processed
    mask: u16,
    //as the number of flipped bits in the mask
    transformations: Vec<i32>,
) {
    data.iter_mut()
        .enumerate()
        .for_each(|(a, b)| *b = a as u128);
    // println!("{:?}", data);
    println!("{:#b}", mask);
    let pa = PosArray::new(mask);
    println!("{:?}", pa);
    for i in pa.unpack().iter().enumerate() {
        if *i.1 {
            println!("{:?}", i.0);
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
struct PosArray<T> {
    repr: T,
}
impl PosArray<u16> {
    fn new(repr: u16) -> Self { Self { repr } }

    fn unpack(&self) -> [bool; 16] {
        [
            self.repr & (1 << 0) != 0,
            self.repr & (1 << 1) != 0,
            self.repr & (1 << 2) != 0,
            self.repr & (1 << 3) != 0,
            self.repr & (1 << 4) != 0,
            self.repr & (1 << 5) != 0,
            self.repr & (1 << 6) != 0,
            self.repr & (1 << 7) != 0,
            self.repr & (1 << 8) != 0,
            self.repr & (1 << 9) != 0,
            self.repr & (1 << 10) != 0,
            self.repr & (1 << 11) != 0,
            self.repr & (1 << 12) != 0,
            self.repr & (1 << 13) != 0,
            self.repr & (1 << 14) != 0,
            self.repr & (1 << 15) != 0,
        ]
    }

    pub fn repr(&self) -> u16 { self.repr }
}
impl From<[bool; 16]> for PosArray<u16> {
    fn from(data: [bool; 16]) -> PosArray<u16> {
        let mut repr: u16 = 0;
        for i in data.iter().enumerate() {
            repr |= (*i.1 as u16) << i.0 as u16;
        }
        PosArray { repr }
    }
}
