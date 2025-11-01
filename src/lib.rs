pub mod chapters;
pub mod compile;

use std::fs;

pub fn compile_docs() -> Result<(), String> {
    match fs::create_dir_all("./docs") {
        Err(e) => return Err(format!("creating dir: {}", e)),
        Ok(_) => (),
    }

    let doc = compile::compile("./src/chapters/closures.rs")?;
    doc.write_to_file("./docs/closures.md")?;

    let doc = compile::compile("./src/chapters/traits.rs")?;
    doc.write_to_file("./docs/traits.md")?;

    Ok(())
}
