use crate::load_translations::Translation;
use std::collections::HashMap;

/// Recursively searches for regex matches in all files within a path
/// Returns a vector of tuples: (file_path, line_number, matched_text)
pub fn map_translations_by_translation(
    translation: &[Translation],
) -> HashMap<String, Vec<&Translation>> {
    let mut hashmap: HashMap<String, Vec<&Translation>> = HashMap::new();

    translation.iter().for_each(|translation| {
        hashmap
            .entry(translation.translations.clone())
            .or_insert_with(Vec::new)
            .push(translation);
    });

    hashmap
}
