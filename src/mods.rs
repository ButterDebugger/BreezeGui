use crate::{config::Config, utils::get_mod_names};
use dialoguer::{theme::ColorfulTheme, Select};
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

pub fn gui(config: Config) {
    let selections = &["List", "Clear"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Regarding your mods, what would you like to do")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        match selection {
            0 => list(config),
            1 => clear(config),
            _ => panic!(),
        }
    } else {
        println!();
        println!("Returning to main menu");
    }
}

fn list(config: Config) {
    // Get the list of installed mods
    let installed_mods = get_mod_names(config);

    if installed_mods.is_empty() {
        println!();
        println!("No mods found");
        return;
    }

    // Print all of the mod names
    println!();
    println!("Installed mods:");
    for mod_name in installed_mods {
        println!("- {}", mod_name);
    }
}

fn clear(config: Config) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Get the list of installed mods
    let installed_mods = get_mod_names(config);

    if installed_mods.is_empty() {
        println!();
        println!("No mods found");
        return;
    }

    // Delete every mod
    for mod_name in installed_mods {
        let mod_file_name = mod_name + ".jar";
        let _ = fs::remove_file(minecraft_path.join("mods").join(mod_file_name.clone()));
    }

    println!();
    println!("Mods have been successfully cleared!");
}
