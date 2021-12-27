use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundImage {
    kind: BackgroundImage,
}

#[derive(Clone, Debug)]
enum BackgroundImage {
    Standard(String),
    Arbitrary(TailwindArbitrary),
    // From(AnchorPoint),
    // To(AnchorPoint),
}

impl Display for TailwindBackgroundImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bg-")?;
        match &self.kind {
            BackgroundImage::Standard(s) => match s.as_str() {
                "none" => write!(f, "none"),
                _ => write!(f, "image-{}", s),
            },
            BackgroundImage::Arbitrary(s) => write!(f, "image-{}", s.get_class()),
        }
    }
}

impl TailwindInstance for TailwindBackgroundImage {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let clip = match &self.kind {
            BackgroundImage::Standard(s) => s.to_string(),
            BackgroundImage::Arbitrary(s) => s.get_properties(),
        };
        css_attributes! {
            "background-image" => clip
        }
    }
}

impl TailwindBackgroundImage {
    /// <https://tailwindcss.com/docs/background-image>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: BackgroundImage::parse(pattern, arbitrary)? })
    }
    /// <https://tailwindcss.com/docs/background-image#arbitrary-values>
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: BackgroundImage::parse_arbitrary(arbitrary)? })
    }
}

impl BackgroundImage {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let out = match pattern {
            [] => Self::parse_arbitrary(arbitrary)?,
            [s] if Self::check_valid(s) => Self::Standard(s.to_string()),
            _ => return syntax_error!("Unknown bg-image instructions: {}", pattern.join("-")),
        };
        Ok(out)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-origin#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "none", "revert", "unset"]);
        set.contains(mode)
    }
}
