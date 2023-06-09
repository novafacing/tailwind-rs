use lightningcss::{stylesheet::ParserFlags, targets::Targets};
use tailwind_error::TailwindError;

use super::*;
use crate::CLIConfig;

impl CLIConfig {
    pub fn compile_css(&self, css: &str) -> Result<String> {
        let parser: ParserOptions = ParserOptions {
            //
            filename: "".to_string(),
            css_modules: None,
            source_index: 0,
            error_recovery: false,
            warnings: None,
            flags: ParserFlags::NESTING | ParserFlags::CUSTOM_MEDIA,
        };
        let mut stylesheet = StyleSheet::parse(css, parser)
            .map_err(|e| TailwindError::runtime_error(format!("Error parsing stylesheet: {}", e)))?;
        let minify = MinifyOptions { targets: Targets::default(), unused_symbols: self.unused_symbols.to_owned() };
        stylesheet.minify(minify).map_err(|e| TailwindError::runtime_error(format!("Error minifying: {}", e)))?;
        let printer = PrinterOptions {
            //
            minify: self.minify,
            source_map: None,
            targets: None.into(),
            analyze_dependencies: None,
            pseudo_classes: None,
            project_root: None,
        };
        let css =
            stylesheet.to_css(printer).map_err(|e| TailwindError::runtime_error(format!("Error converting to CSS: {}", e)))?;
        Ok(css.code)
    }
}
