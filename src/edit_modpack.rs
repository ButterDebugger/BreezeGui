use crate::{config::Config, utils::has_mods};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use libium::modpack::zip_extract;
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

pub fn gui(config: Config, modpack_name: String) {
    let selections = &["Load", "Rename", "Delete"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do to ".to_owned() + &modpack_name)
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        match selection {
            0 => load(config, modpack_name),
            1 => rename(config, modpack_name),
            2 => delete(config, modpack_name),
            _ => panic!(),
        }
    } else {
        println!();
        println!("Returning to main menu");
    }
}

fn load(config: Config, modpack_name: String) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    let modpack_file_name = modpack_name.clone() + ".zip";

    // Cancel if the user already has mods installed
    if has_mods(config) {
        println!();
        println!("You cannot load a modpack with mods already installed");
        return;
    }

    // Extract the file into the mods dir
    let modpack_path = minecraft_path
        .join("modpacks")
        .join(modpack_file_name.clone());
    let mods_path = minecraft_path.join("mods");

    let _ = zip_extract(&modpack_path, &mods_path);

    println!();
    println!("{} has successfully been loaded!", modpack_name)
}

fn rename(config: Config, modpack_name: String) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    let modpack_file_name = modpack_name.clone() + ".zip";

    // Get the new name
    let new_name: String = Input::new()
        .with_prompt("What is the new name of this modpack?")
        .interact_text()
        .unwrap();

    let new_file_name = new_name.clone() + ".zip";

    // Rename the modpack
    let old_path = minecraft_path
        .join("modpacks")
        .join(modpack_file_name.clone());
    let new_path = minecraft_path.join("modpacks").join(new_file_name.clone());
    let _ = fs::rename(old_path, new_path);

    println!();
    println!(
        "{} has successfully been renamed to {}",
        modpack_name, new_name
    )
}

fn delete(config: Config, modpack_name: String) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    let modpack_file_name = modpack_name.clone() + ".zip";

    // Delete the modpack
    let modpack_path = minecraft_path
        .join("modpacks")
        .join(modpack_file_name.clone());
    let _ = fs::remove_file(modpack_path);

    println!();
    println!("{} has successfully been deleted!", modpack_name)
}
