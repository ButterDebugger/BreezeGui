use ferinth::Ferinth;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct HashCache()

#[derive(Debug, Clone)]
pub struct DownloadsConfig {
    pub cache_dir: PathBuf,
    pub hash_cache_path: PathBuf,
}
