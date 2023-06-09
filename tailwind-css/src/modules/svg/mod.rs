pub use self::{
    fill::TailwindFillColor,
    stroke::{stroke_color::TailwindStrokeColor, stroke_width::TailwindStrokeWidth, TailwindStroke},
};
use crate::{
    css_attributes, CssAttributes, LengthUnit, Result, TailwindArbitrary, TailwindBuilder, TailwindColor, TailwindInstance,
};
use std::fmt::{Display, Formatter};

mod fill;
mod stroke;
