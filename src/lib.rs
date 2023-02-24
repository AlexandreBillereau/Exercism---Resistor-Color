use std::fmt::Display;
use int_enum::IntEnum;
use enum_iterator::{Sequence};

#[repr(u32)]
#[derive(Debug, Copy, Clone ,PartialEq, Eq, IntEnum, Sequence, PartialOrd, Ord)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl Display for ResistorColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
   let color = match ResistorColor::from_int(value) {
        Ok(x) => x.to_string(), 
        _  => "value out of range".to_string(),
   };

   return color 
}

pub fn colors() -> Vec<ResistorColor> {
    let mut test = enum_iterator::all::<ResistorColor>().collect::<Vec<_>>();
    test.sort();
    return test;
}
