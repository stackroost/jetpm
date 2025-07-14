use crate::core::resolve::resolve_global_import;
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn rewrite_imports(input: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let code = fs::read_to_string(input)?;
    let import_regex_single =
        Regex::new(r#"(\s*import\s+.*?\s+from\s+)'(?P<specifier>[^./][^']*)'"#)?;
    let import_regex_double =
        Regex::new(r#"(\s*import\s+.*?\s+from\s+)"(?P<specifier>[^./][^"]*)""#)?;

    let rewritten =
        import_regex_single.replace_all(&code, |caps: &regex::Captures| rewrite_import(caps));

    let rewritten =
        import_regex_double.replace_all(&rewritten, |caps: &regex::Captures| rewrite_import(caps));

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output, rewritten.as_bytes())?;
    Ok(())
}

fn rewrite_import(caps: &regex::Captures) -> String {
    let import_stmt = caps.get(1).unwrap().as_str();
    let bare_specifier = caps.name("specifier").unwrap().as_str();

    match resolve_global_import(bare_specifier, None) {
        Some(resolved) => {
            if let Ok(abs_path) = resolved.canonicalize() {
                let path_url = format!("file://{}", abs_path.to_string_lossy().replace('\\', "/"));
                format!("{}'{}'", import_stmt, path_url)
            } else {
                caps[0].to_string()
            }
        }
        None => caps[0].to_string(),
    }
}
