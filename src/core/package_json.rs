// use std::fs;
// use serde_json::Value;
// use std::path::Path;

// pub fn get_script_from_dir(dir: &Path, script: &str) -> Option<String> {
//     let pkg_path = dir.join("package.json");
//     let content = fs::read_to_string(pkg_path).ok()?;
//     let json: Value = serde_json::from_str(&content).ok()?;
//     json.get("scripts")?
//         .get(script)?
//         .as_str()
//         .map(|s| s.to_string())
// }