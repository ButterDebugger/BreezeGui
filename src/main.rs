use breeze_tui::App;

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

    let mut app = App::default();

    app.run().await;
}
