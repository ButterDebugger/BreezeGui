use crate::format::{Branch, ModLoader};
use crate::{format::Mod, platforms::downloads::DownloadsConfig, MODRINTH_API};
use anyhow::Result;
use ferinth::structures::version::{Dependency, DependencyType};
use reqwest::Url;
use std::fs::{create_dir_all, File};
use std::io::Write;

#[derive(Debug, Clone)]
pub enum ModEntry {
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
    mod_id: String,
    filename: String,
    url: Url,
}

pub async fn get_required_versions(
    game_version: String,
    mod_loader: ModLoader,
    mods: Vec<ModEntry>,
) -> Result<Vec<ModFile>> {
    let mut mods = mods.clone();
    let mut downloads = Vec::new();

    while let Some(mc_mod) = mods.pop() {
        match mc_mod {
            ModEntry::Modrinth {
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
                                mods.push(ModEntry::Modrinth {
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
            ModEntry::CurseForge { .. } => {
                unimplemented!("CurseForge mod version fetching is not implemented yet");
            }
        }
    }

    Ok(downloads)
}

pub async fn download_mod(downloads: DownloadsConfig, mc_mod: Mod) -> Result<()> {
    match mc_mod {
        Mod::Modrinth {
            project_id,
            version,
            ..
        } => {
            let version = MODRINTH_API
                .version_get_from_number(project_id.as_str(), version.as_str())
                .await?;

            println!("Version: {:#?}", version);

            if let Some(version_file) = version.files.iter().find(|e| e.primary) {
                println!("Downloading {} from Modrinth...", version_file.filename);

                let response = reqwest::get(version_file.url.clone()).await?;
                let bytes = response.bytes().await?;

                create_dir_all(&downloads.cache_dir)?;

                let dest_path = downloads.cache_dir.join(version_file.filename.clone());
                let mut dest = File::create(dest_path)?;

                dest.write_all(&bytes)?;
            } else {
                println!("No primary file found for this version.");
            }
        }
        Mod::CurseForge { .. } => {
            unimplemented!("CurseForge mod downloading is not implemented yet");
        }
    }

    Ok(())
}
