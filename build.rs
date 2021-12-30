use std::path::Path;
use std::{env, fs};

use anyhow::{anyhow, Result};
use regex::Regex;
use voca_rs::case;

fn main() -> Result<()> {
    let mut hooks = vec!["&[\n".into()];

    for entry in fs::read_dir("src/exports")? {
        let entry = entry?;
        let path = entry.path();
        let is_file = path.is_file();
        let extension = path.extension();

        if is_file && matches!(extension, Some(ext) if ext == "rs") {
            let content = fs::read_to_string(&path)?;

            for line in content.split("\n") {
                if line.contains("export_name") {
                    let reg_expr = Regex::new(r#"(?m)["](.*)["]"#)?;
                    let reg_match = reg_expr
                        .find(&line)
                        .ok_or_else(|| anyhow!("Regex doesn't match!"))?;

                    let name = &line[reg_match.start()..reg_match.end()];
                    let rust_name = case::snake_case(&name);

                    hooks.push(format!(
                        "\t({}, DetourPtr({} as * const())),\n",
                        &name, &rust_name
                    ));
                }
            }
        }
    }

    hooks.push("]".into());

    let dir = env::var("OUT_DIR")?;
    let path = Path::new(&dir).join("hooks.rs");
    let data = hooks.join("");

    fs::write(&path, &data)?;
    Ok(())
}
