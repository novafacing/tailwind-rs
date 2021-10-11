use super::*;

#[derive(Clone, Debug)]
pub struct TailwindRight {
    negative: bool,
    kind: PlacementSize,
}

impl Display for TailwindRight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "right-{}", self.kind)
    }
}

impl TailwindInstance for TailwindRight {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        css_attributes! {
            "right" => self.kind.get_properties()
        }
    }
}

impl TailwindRight {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse(kind, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, negative: bool) -> Result<Self> {
        Ok(Self { negative, kind: PlacementSize::parse_unit(arbitrary)? })
    }
}
