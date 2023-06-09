use super::*;

impl Display for TailwindInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for v in &self.variants {
            write!(f, "{}", v)?
        }
        self.negative.write(f)?;
        match self.arbitrary.is_some() {
            true => write!(f, "{}-{}", self.elements, self.arbitrary.get_class()),
            false => write!(f, "{}", self.elements),
        }
    }
}

impl TailwindInstance for TailwindInstruction {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        let mut out = CssAttributes::default();
        match self.get_instance() {
            Ok(o) => {
                out += o.attributes(ctx);
            },
            Err(e) => {
                unimplemented!("{}", e);
            },
        }
        out
    }
}

impl Display for TailwindVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.not {
            write!(f, "not-")?
        }
        write!(f, "{}", self.names.join("-"))?;
        match self.pseudo {
            true => {
                write!(f, "::")
            },
            false => {
                write!(f, ":")
            },
        }
    }
}

impl Display for TailwindElements {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.join("-"))
    }
}
