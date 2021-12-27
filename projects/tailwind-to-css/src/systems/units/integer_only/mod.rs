use super::*;
mod traits;

/// Used to represent those attributes that only have integers
#[derive(Debug, Clone)]
pub enum IntegerOnly {
    Number(usize),
    Arbitrary(TailwindArbitrary),
}

impl IntegerOnly {
    pub fn parser(id: &'static str) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary| match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            [n] => Ok(Self::Number(TailwindArbitrary::from(*n).as_integer()?)),
            _ => Err(TailwindError::syntax_error(format!("Unknown {} pattern", id))),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
}
