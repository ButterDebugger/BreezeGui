use breeze_pack::{
    format::{Branch, Mod, ModLoader, Modpack},
    packer::Packer,
    platforms::{
        downloads::DownloadsConfig,
        versions::{get_required_versions, ModSource},
    },
};
use breeze_tui::App;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() {
    // let _ = download_mod(
    //     DownloadsConfig {
    //         cache_dir: "cache".into(),
    //         hash_cache_path: "hash_cache.json".into(),
    //     },
    //     Mod::Modrinth {
    //         name: "Sodium".to_owned(),
    //         project_id: "AANobbMI".to_owned(),
    //         version: "uGvVQBnw".to_owned(),
    //     },
    // )
    // .await;

    // println!(
    //     "{:#?}",
    //     get_required_versions(
    //         "1.21.11".to_owned(),
    //         ModLoader::Fabric,
    //         vec![
    //             ModEntry::Modrinth {
    //                 project_id: "AANobbMI".to_owned(),
    //                 // version: Some("UddlN6L4".to_owned()),
    //                 version: None,
    //             },
    //             ModEntry::Modrinth {
    //                 project_id: "iris".to_owned(),
    //                 version: None,
    //             },
    //         ],
    //     )
    //     .await
    // );

    // let pack = Packer::new(PathBuf::from("./wha.modx").into());

    // let _ = pack.write_pack(&Modpack {
    //     name: "nam".to_string(),
    //     summary: None,
    //     author: None,
    //     updater: None,
    //     branches: vec![],
    // });

    // let _ = pack.write_branch(
    //     "what",
    //     &Branch {
    //         game_version: "1.21.11".to_string(),
    //         mod_loader: ModLoader::Fabric,
    //         loader_version: None,
    //         mods: vec![Mod {
    //             name: "Sodium".to_owned(),
    //             source: ModSource::Modrinth {
    //                 project_id: "AANobbMI".to_owned(),
    //                 version: Some("uGvVQBnw".to_owned()),
    //             },
    //         }],
    //     },
    // );

    // let _ = pack.save();

    let mut app = App::default();

    app.run().await;
}
