use super::*;

mod builder;
mod display;

#[doc = include_str ! ("aspect-ratio.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundColor {
    pub(crate) color: ColorResolver,
}

#[doc = include_str ! ("aspect-ratio.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundBrightness {
    brightness: TailwindBrightness,
}
