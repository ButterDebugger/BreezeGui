use crate::platforms::versions::ModSource;
use crate::MODRINTH_API;
use anyhow::Result;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct HashCache()

#[derive(Debug, Clone)]
pub struct DownloadsConfig {
    pub cache_dir: PathBuf,
    pub hash_cache_path: PathBuf,
}

pub async fn download_mod(downloads: DownloadsConfig, mc_mod: ModSource) -> Result<()> {
    match mc_mod {
        ModSource::Modrinth {
            project_id,
            version,
        } => {
            let version = if let Some(version) = version {
                MODRINTH_API
                    .version_get_from_number(project_id.as_str(), version.as_str())
                    .await?
            } else {
                unimplemented!("Downloading a mod without a version is not implemented yet")
            };

            println!("Version: {:#?}", version);

            if let Some(version_file) = version.files.iter().find(|e| e.primary) {
                println!("Downloading {} from Modrinth...", version_file.filename);

                // Fetch the file
                let response = reqwest::get(version_file.url.clone()).await?;

                let total_size = response
                    .content_length()
                    .unwrap_or(version_file.size.try_into().unwrap());

                // Create progress bar
                let pb = ProgressBar::new(total_size);

                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                        .unwrap()
                        .progress_chars("#>-")
                        .tick_strings(&[
                            "⠁", "⠂", "⠄", "⡀", "⡈", "⡐", "⡠", "⣀", "⣁", "⣂", "⣄", "⣌", "⣔", "⣤", "⣥", "⣦",
                            "⣮", "⣶", "⣷", "⣿", "⡿", "⠿", "⢟", "⠟", "⡛", "⠛", "⠫", "⢋", "⠋", "⠍", "⡉", "⠉",
                            "⠑", "⠡", "⢁", "✔",
                        ]),
                );

                // Download the file
                let mut bytes = Vec::with_capacity(total_size as usize);
                let mut stream = response.bytes_stream();

                while let Some(chunk_result) = stream.next().await {
                    let chunk = chunk_result?;

                    bytes.extend(&chunk);

                    pb.inc(chunk.len() as u64);
                }

                pb.finish_with_message("Download finished!");

                // Output the file into the cache directory
                create_dir_all(&downloads.cache_dir)?;

                let dest_path = downloads.cache_dir.join(version_file.filename.clone());
                let mut dest = File::create(dest_path)?;

                dest.write_all(&bytes)?;
            } else {
                println!("No primary file found for this version.");
            }
        }
        ModSource::CurseForge { .. } => {
            unimplemented!("CurseForge mod downloading is not implemented yet");
        }
    }

    Ok(())
}
