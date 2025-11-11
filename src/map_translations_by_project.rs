use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use once_cell::sync::Lazy;
use crate::entities::PackageType;
use crate::load_translations::Translation;

pub fn map_translations_by_project(
    translation: &[Translation],
) -> HashMap<String, Vec<&Translation>> {
    let mut hashmap: HashMap<String, Vec<&Translation>> = HashMap::new();

    translation.iter().for_each(|translation| {
        hashmap
            .entry(get_project_path(translation.path.to_str().unwrap()))
            .or_insert_with(Vec::new)
            .push(translation);
    });

    hashmap
}

static PROJECT_PATH_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(packages/manager/(apps|modules)/[^/]+)").unwrap()
});

pub(crate) fn determinate_project_path_and_type(path: &str) -> Option<(PackageType, String)> {
    if let Some(caps) = PROJECT_PATH_REGEX.captures(path) {
        let identifier = caps.get(1)?.as_str().to_string();
        let pkg_type = PackageType::from_str(caps.get(2)?.as_str()).unwrap_or(PackageType::Modules);
        return Some((pkg_type, identifier));
    }
    None
}

pub(crate) fn get_project_path(path: &str) -> String {
    determinate_project_path_and_type(path).map_or_else(|| "unknown".to_string(), |package| package.1)
}
