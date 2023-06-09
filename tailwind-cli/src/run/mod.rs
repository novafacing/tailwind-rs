use byte_unit::Byte;
use fs::read_to_string;
use std::{
    env::{current_dir, set_current_dir},
    fs,
    path::Path,
};

use clap::ValueEnum;
use glob::glob;
use tailwind_error::TailwindError;

use tailwind_rs::{CLIConfig, CssInlineMode, Result, TailwindBuilder};

use crate::TailwindApp;

impl TailwindApp {
    pub fn build_config(&self) -> (CLIConfig, TailwindBuilder) {
        let mut config = CLIConfig {
            mode: match self.mode {
                Some(Mode::Inline) => CssInlineMode::Inline,
                Some(Mode::Scope) => CssInlineMode::Scoped,
                Some(Mode::Key) => CssInlineMode::DataKey,
                Some(Mode::Value) => CssInlineMode::DataValue,
                _ => CssInlineMode::None,
            },
            ..Default::default()
        };
        if let Some(s) = self.minify {
            config.minify = s;
        }
        config.dry_run = self.dry_run;
        let builder = config.builder();
        if let Some(s) = self.obfuscate {
            config.obfuscate = s;
        }
        self.set_workspace().ok();
        // set_current_dir()
        (config, builder)
    }
    fn set_workspace(&self) -> Result<()> {
        if let Some(s) = &self.workspace {
            set_current_dir(s)?;
        }
        println!("Current workspace: {:?}", current_dir()?);
        Ok(())
    }
}

impl TailwindApp {
    pub fn run(&self, config: &CLIConfig, builder: &mut TailwindBuilder) -> Result<()> {
        if let Some(c) = &self.command {
            return c.run(config);
        };

        for entry in glob("**/*.html")
            .map_err(|e| TailwindError::runtime_error(format!("Error getting paths from html files: {}", e)))?
        {
            let file = entry.map_err(|e| TailwindError::runtime_error(format!("Error getting path: {}", e)))?;
            let input = read_to_string(&file)?;
            let ext = get_extension(&file).ok_or_else(|| TailwindError::runtime_error("no extension"))?;
            match ext {
                "html" => {
                    let (html, css) = match config.compile_html(&input, builder) {
                        Ok(o) => o,
                        Err(e) => {
                            println!("{}", file.display());
                            println!("{}", e);
                            continue;
                        },
                    };
                    if config.dry_run {
                        let html = Byte::from(html.len()).get_appropriate_unit(false);
                        let css = Byte::from(css.len()).get_appropriate_unit(false);
                        println!("dry run on {} success", file.display());
                        println!("HTML size: {}, Css Size: {}", html, css);
                        continue;
                    }
                },
                _ => {
                    todo!("unsupported format {}", ext);
                },
            }
        }
        Ok(())
    }
}

fn get_extension(path: &Path) -> Option<&str> {
    path.extension()?.to_str()
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Normal,
    Inline,
    Scope,
    Key,
    Value,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Normal
    }
}
