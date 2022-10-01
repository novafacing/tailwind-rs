use super::*;
use crate::{TailwindBuilder, TailwindInstance};
use std::collections::BTreeSet;

mod traits;

/// Used to represent those attributes that only have keywords
#[derive(Debug, Clone)]
pub enum StandardValue {
    Keyword(String),
    Arbitrary(TailwindArbitrary),
}

#[derive(Debug, Clone)]
pub struct KeywordInstance {
    pub pattern: &'static str,
    pub value: StandardValue,
}

impl KeywordInstance {
    pub fn parse<'i>(
        id: &'static str,
        pattern: &'i [&'i str],
        valid: &'static [&'static str],
        arbitrary: &'i TailwindArbitrary,
    ) -> Result<Self> {
        match pattern {
            [] => {
                let value = StandardValue::Arbitrary(TailwindArbitrary::new(arbitrary)?);
                Ok(Self { pattern: id, value })
            },
            _ => {
                let keyword = pattern.join("-");
                let checker = BTreeSet::from_iter(valid);
                if cfg!(compile_time) && !checker.contains(&&*keyword) {
                    return syntax_error!("{} does not a valid value of {}", keyword, id);
                }
                Ok(Self { pattern: id, value: StandardValue::Keyword(keyword) })
            },
        }
    }
}

impl StandardValue {
    pub fn parser(id: &'static str, check_valid: &'static impl Fn(&str) -> bool) -> impl Fn(&[&str], &TailwindArbitrary) -> Result<Self> {
        move |pattern: &[&str], arbitrary: &TailwindArbitrary| match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            _ => Self::parse_keyword(pattern, id, check_valid),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
    pub fn parse_keyword(pattern: &[&str], id: &str, checker: &'static impl Fn(&str) -> bool) -> Result<Self> {
        let keyword = pattern.join("-");
        if cfg!(compile_time) && !checker(&keyword) {
            return syntax_error!("{} does not a valid value of {}", keyword, id);
        }
        Ok(Self::Keyword(keyword))
    }
    pub fn get_properties(&self) -> &str {
        match self {
            Self::Keyword(s) => s.as_str(),
            Self::Arbitrary(s) => s.as_str(),
        }
    }
    pub fn get_value(&self) -> &str {
        match self {
            Self::Keyword(s) => s.as_str(),
            Self::Arbitrary(s) => s.as_str(),
        }
    }
    pub fn write_class(&self, fmt: &mut Formatter, class: &str, special: fn(&mut Formatter, &str) -> std::fmt::Result) -> std::fmt::Result {
        match self {
            StandardValue::Keyword(s) => match special(fmt, s) {
                Ok(o) => Ok(o),
                Err(_) => write!(fmt, "{}", class),
            },
            StandardValue::Arbitrary(s) => s.write_class(fmt, class),
        }
    }
}
