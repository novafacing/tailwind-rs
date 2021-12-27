use super::*;
use crate::StandardValue;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindOverflow {
    kind: StandardValue,
    axis: Option<bool>,
}

impl Display for TailwindOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.axis {
            None => write!(f, "overflow-{}", self.kind),
            Some(true) => write!(f, "overflow-x-{}", self.kind),
            Some(false) => write!(f, "overflow-y-{}", self.kind),
        }
    }
}

impl TailwindInstance for TailwindOverflow {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = match self.axis {
            None => "overflow",
            Some(true) => "overflow-x",
            Some(false) => "overflow-y",
        };
        css_attributes! {
            class => self.kind
        }
    }
}

impl TailwindOverflow {
    /// https://tailwindcss.com/docs/overflow#header
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, axis: Option<bool>) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("overflow", &Self::check_valid)(pattern, arbitrary)?, axis })
    }
    /// https://tailwindcss.com/docs/font-variant-numeric#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: Option<bool>) -> Result<Self> {
        Ok(Self { kind: StandardValue::parse_arbitrary(arbitrary)?, axis })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/overflow#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            // Keyword values
            "visible", "hidden", "clip", "scroll", "auto", // Global  values
            "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
