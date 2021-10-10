use super::*;

impl SizingUnit {
    pub fn parse(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let px = |x| Ok(Self::Length(LengthUnit::em(x)));
        match kind {
            ["min"] => Ok(Self::Min),
            ["max"] => Ok(Self::Max),
            ["auto"] => Ok(Self::Auto),
            ["full"] => Ok(Self::Full),
            ["0"] => px(0.0),
            ["px"] => px(1.0),
            [n] => Self::parse_arbitrary(&TailwindArbitrary::from(*n)),
            [] => Self::parse_arbitrary(arbitrary),
            _ => syntax_error!("Unknown sizing instructions: {}", kind.join("-")),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::maybe_fraction(arbitrary).or_else(|_| Self::maybe_no_unit(arbitrary)).or_else(|_| Self::maybe_length(arbitrary))
    }
    #[inline]
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length()?))
    }
    #[inline]
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let rem = |x| Ok(Self::Length(LengthUnit::em(x)));
        rem(arbitrary.as_float()? / 4.0)
    }
    #[inline]
    fn maybe_fraction(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (a, b) = arbitrary.as_fraction()?;
        Ok(Self::Fraction(a, b))
    }
}

impl TailwindSizing {
    #[inline]
    pub fn parse_width(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::Width, size: SizingUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_width_max(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MaxWidth, size: SizingUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_width_min(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MinWidth, size: SizingUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_height(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::Height, size: SizingUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_height_max(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MaxHeight, size: SizingUnit::parse(kind, arbitrary)? })
    }
    #[inline]
    pub fn parse_height_min(kind: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MinHeight, size: SizingUnit::parse(kind, arbitrary)? })
    }
}
