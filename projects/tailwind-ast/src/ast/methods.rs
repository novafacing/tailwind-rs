use super::*;

impl AstStyle {
    /// Expand all group to simple styles
    ///
    /// ```tw
    /// text(red blue)
    ///   - text-red
    ///   - text-blue
    /// ```
    pub(crate) fn expand_visit(self, parent: &AstStyle) -> AstStyle {
        let mut elements = parent.elements.clone();
        elements.items.extend_from_slice(&self.elements.items);
        elements.negative = merge_negative(elements.negative, self.elements.negative);
        AstStyle {
            important: merge_important(self.important, parent.important),
            variants: self.variants.clone(),
            elements,
            arbitrary: self.arbitrary.clone(),
            children: self.children.clone(),
        }
    }
}

impl AstArbitrary {
    ///
    #[inline]
    pub fn as_class(&self) -> String {
        self.item.to_string()
    }
    ///
    #[inline]
    pub fn as_str(&self) -> &str {
        self.item.as_str()
    }
    /// Check if the arbitrary is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.item.is_empty()
    }
}

impl ASTVariant {
    /// TODO: `&[&str]]`
    pub fn as_view(&self) -> Vec<&str> {
        self.names.iter().map(AsRef::as_ref).collect()
    }
}

#[inline]
fn merge_important(lhs: bool, rhs: bool) -> bool {
    lhs || rhs
}

#[inline]
#[allow(clippy::match_like_matches_macro)]
fn merge_negative(lhs: bool, rhs: bool) -> bool {
    match (lhs, rhs) {
        (true, true) => false,
        (false, false) => false,
        _ => true,
    }
}
