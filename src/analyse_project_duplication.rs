use std::collections::HashMap;
use crate::entities::Translation;

#[derive(PartialEq, Debug)]
enum DuplicationType {
    InterPackage,
    CommonTranslation,
    ExternalProjects
}
pub struct DuplicationReport<'a> {
    translation: &'a Translation,
    duplicationType: DuplicationType
}

pub fn analyse_duplication<'a>(project_path: &str, translations_to_check: &[&'a Translation], all_translations: &HashMap<String, Vec<&Translation>>) -> Vec<DuplicationReport<'a>> {
    let mut duplications: Vec<DuplicationReport<'a>> = Vec::new();
    for translation in translations_to_check {
        let translations_found = all_translations.get(&translation.translations).unwrap();

        if translations_found.len() == 1 {
            continue
        }

        if translations_found.iter().find(|t| t.path.to_string_lossy().to_string().contains("common-translations")).is_some() {
            duplications.push(DuplicationReport { translation, duplicationType: DuplicationType::CommonTranslation });
            continue
        }

        if translations_found.iter().filter(|t| t.path.to_string_lossy().to_string().contains(project_path)).count() > 1 {
            duplications.push(DuplicationReport { translation, duplicationType: DuplicationType::InterPackage });
            continue
        }

    duplications.push(DuplicationReport { translation, duplicationType: DuplicationType::ExternalProjects });
    }

    duplications
}

pub fn print_global_duplication_report(duplications: &[DuplicationReport]) {
    let count_inter_duplication = duplications.iter().filter(|duplication| duplication.duplicationType == DuplicationType::InterPackage).count();
    let count_common_duplication = duplications.iter().filter(|duplication| duplication.duplicationType == DuplicationType::CommonTranslation).count();
    let count_external_duplication = duplications.iter().filter(|duplication| duplication.duplicationType == DuplicationType::ExternalProjects).count();

    println!("Global duplication report :");
    println!("Inter-package duplication : {}", count_inter_duplication);
    println!("Common-translation duplication : {}", count_common_duplication);
    println!("External-projects duplication : {}", count_external_duplication);
    println!("Total duplication : {}", count_inter_duplication + count_common_duplication + count_external_duplication);

    // println!("\n");
    // for duplication in duplications {
    //     println!("{} - {} - {:?}", duplication.translation.path.to_string_lossy(), duplication.translation.key, duplication.duplicationType)
    // }
    // 
    // println!("\n\n");
}