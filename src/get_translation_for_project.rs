use crate::entities::Translation;
use crate::map_translations_by_project::get_package_path;

pub fn get_translations_for_project<'a>(
    project_path: &str,
    translation: &'a [Translation],
) -> Vec<&'a Translation> {
    let mut translations: Vec<&'a Translation> = Vec::new();

    translation.iter().for_each(|translation| {
        if get_package_path(translation.path.to_str().unwrap()) == project_path {
            translations.push(translation);
        }
    });

    translations
}