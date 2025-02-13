use anyhow::{anyhow, Context, Result};
use predicate::str::contains;
use predicates::prelude::*;
use std::path::PathBuf;
use std::process::Command;

pub struct DrawioDesktop<'a> {
    application: &'a str,
    is_headless: bool,
}

impl<'a> DrawioDesktop<'a> {
    pub fn new(application: &String, is_headless: bool) -> Result<DrawioDesktop<'_>> {
        if !PathBuf::from(application).exists() {
            return Err(anyhow!(format!(
                "Draw.io Desktop application path '{}' don't exists",
                application
            )));
        }

        Ok(DrawioDesktop {
            application,
            is_headless,
        })
    }

    pub fn execute(&self, arguments: ExportArguments<'a>) -> Result<()> {
        let mut shell_arguments = arguments.as_shell_arguments();

        if self.is_headless {
            shell_arguments.push("--no-sandbox");
            shell_arguments.push("--disable-dev-shm-usage");
        }

        let command_output = Command::new(self.application)
            .args(&shell_arguments)
            .output()
            .with_context(|| {
                format!(
                    "failed to execute application command line {} {}",
                    self.application,
                    shell_arguments.join(" ")
                )
            })?;

        if let Ok(command_output_string) = String::from_utf8(command_output.stdout.clone()) {
            if !command_output.status.success()
                || contains("Error: ").eval(command_output_string.as_str())
            {
                let stderr = match String::from_utf8(command_output.stderr) {
                    Ok(output) => output,
                    Err(err) => format!("unreadable output due to {}", err),
                };
                anyhow::bail!("fail to export using draw.io desktop\n{}", stderr.as_str());
            }
        }
        Ok(())
    }
}

pub fn os_default_application<'a>() -> &'a str {
    match std::env::consts::OS {
        "macos" => "/Applications/draw.io.app/Contents/MacOS/draw.io",
        "windows" => "C:\\Program Files\\draw.io\\draw.io.exe",
        "linux" => "/opt/drawio/drawio",
        _ => "",
    }
}

pub struct ExportArguments<'a> {
    pub recursive: bool,
    pub output: Option<&'a str>,
    pub input: &'a str,
    pub format: &'a str,
    pub border: &'a String,
    pub scale: Option<&'a String>,
    pub width: Option<&'a String>,
    pub height: Option<&'a String>,
    pub crop: bool,
    pub embed_diagram: bool,
    pub transparent: bool,
    pub quality: &'a String,
    pub uncompressed: bool,
    pub all_pages: bool,
    pub page_index: Option<&'a String>,
    pub page_range: Option<&'a String>,
    pub embed_svg_images: bool,
    pub enable_plugins: bool,
    pub raw: Option<&'a String>,
}

impl<'a> ExportArguments<'a> {
    fn as_shell_arguments(&self) -> Vec<&'a str> {
        // Export Options as shell arguments
        let mut arguments = vec!["--export"];

        if self.recursive {
            arguments.push("--recursive");
        }

        if let Some(raw) = self.raw {
            arguments.push(raw);
        }

        if let Some(output) = self.output {
            arguments.push("--output");
            arguments.push(output);
        }

        arguments.push("--format");
        arguments.push(self.format);

        arguments.push("--quality");
        arguments.push(self.quality);

        if self.transparent {
            arguments.push("--transparent");
        }

        if self.embed_diagram {
            arguments.push("--embed-diagram");
        }

        arguments.push("--border");
        arguments.push(self.border);

        if let Some(scale) = self.scale {
            arguments.push("--scale");
            arguments.push(scale);
        }

        if let Some(width) = self.width {
            arguments.push("--width");
            arguments.push(width);
        }

        if let Some(height) = self.height {
            arguments.push("--height");
            arguments.push(height);
        }

        if self.crop {
            arguments.push("--crop");
        }

        if self.all_pages {
            arguments.push("--all-pages");
        }

        if let Some(page_index) = self.page_index {
            arguments.push("--page-index");
            arguments.push(page_index);
        }

        if let Some(page_range) = self.page_range {
            arguments.push("--page-range");
            arguments.push(page_range);
        }

        if self.uncompressed {
            arguments.push("--uncompressed");
        }

        if self.embed_svg_images {
            arguments.push("--embed-svg-images");
        }

        if self.enable_plugins {
            arguments.push("--enable-plugins");
        }

        // Input is always the last argument
        arguments.push(self.input);

        arguments
    }
}
