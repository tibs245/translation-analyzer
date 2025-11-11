use std::path::PathBuf;
use std::str::FromStr;
use serde::{Deserialize, Serialize};


fn serialize_path_lossy<S>(path: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&path.to_string_lossy())
}

#[derive(Clone, Debug, Serialize)]
pub struct Translation {
    #[serde(serialize_with = "serialize_path_lossy")]
    pub path: PathBuf,
    pub translations: String,
    pub key: String,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageType {
    Apps,
    Modules,
}

impl FromStr for PackageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "apps" => Ok(PackageType::Apps),
            "modules" => Ok(PackageType::Modules),
            _ => Err(format!("Invalid package type: {}", s)),
        }
    }
}