use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NpmMetadata {
    #[serde(rename = "dist-tags")]
    dist_tags: DistTags,
    versions: serde_json::Value,
    description: Option<String>,
    homepage: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DistTags {
    latest: String,
}

pub fn info_command(pkg: &str) {
    let url = format!("https://registry.npmjs.org/{}", pkg);

    let response = match ureq::get(&url).call() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to fetch package metadata: {}", e);
            return;
        }
    };

    let json: NpmMetadata = match response.into_json::<NpmMetadata>() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return;
        }
    };

    println!("Package: {}", pkg);
    println!("Latest Version: {}", json.dist_tags.latest);

    let mut versions: Vec<String> = json.versions.as_object()
        .map(|v| v.keys().cloned().collect())
        .unwrap_or_default();

    versions.sort();
    versions.reverse();

    println!("Available Versions:");
    for v in versions.iter().take(10) {
        println!("  - {}", v);
    }

    if let Some(desc) = json.description {
        println!("Description: {}", desc);
    }

    if let Some(home) = json.homepage {
        println!("Homepage: {}", home);
    }
}
