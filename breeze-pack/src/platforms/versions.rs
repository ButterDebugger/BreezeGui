use crate::format::ModLoader;
use crate::MODRINTH_API;
use anyhow::Result;
use ferinth::structures::version::DependencyType;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModSource {
    Modrinth {
        project_id: String,
        version: Option<String>,
    },
    CurseForge {
        project_id: i32,
        file_id: Option<i32>,
    },
}

#[derive(Debug, Clone)]
pub struct ModFile {
    pub mod_id: String,
    pub filename: String,
    pub url: Url,
}

pub async fn get_required_versions(
    game_version: String,
    mod_loader: ModLoader,
    mods: Vec<ModSource>,
) -> Result<Vec<ModFile>> {
    let mut mods = mods.clone();
    let mut downloads = Vec::new();

    while let Some(mc_mod) = mods.pop() {
        match mc_mod {
            ModSource::Modrinth {
                project_id,
                version,
                ..
            } => {
                let version = if let Some(version) = version {
                    MODRINTH_API
                        .version_get_from_number(project_id.as_str(), version.as_str())
                        .await?
                } else {
                    let versions = MODRINTH_API
                        .version_list_filtered(
                            project_id.as_str(),
                            Some(&[&mod_loader.clone().to_api_string()]),
                            Some(&[game_version.as_str()]),
                            None,
                        )
                        .await?;

                    versions.first().unwrap().clone()
                };

                for dependency in version.dependencies {
                    match dependency.dependency_type {
                        DependencyType::Required => {
                            if let Some(project_id) = dependency.project_id.clone() {
                                mods.push(ModSource::Modrinth {
                                    project_id,
                                    version: dependency.version_id,
                                });
                            }
                        }
                        _ => continue,
                    }
                }

                if let Some(version_file) = version.files.iter().find(|e| e.primary) {
                    downloads.push(ModFile {
                        mod_id: project_id.clone(),
                        filename: version_file.filename.clone(),
                        url: version_file.url.clone(),
                    });
                } else {
                    println!("No primary file found for this version.");
                }
            }
            ModSource::CurseForge { .. } => {
                unimplemented!("CurseForge mod version fetching is not implemented yet");
            }
        }
    }

    Ok(downloads)
}
