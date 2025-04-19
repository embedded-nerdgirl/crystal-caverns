use rfd::MessageDialogResult;
use std::fs;

#[allow(dead_code)]

// These were exported to a utility file so that they can be accessed in regions
// outside of main.rs. Everything is intact and unchanged.

pub fn raise_error(title: &str, message: &str) {
    let result = rfd::MessageDialog::new()
        .set_title(title)
        .set_description(message)
        .set_buttons(rfd::MessageButtons::Ok)
        .set_level(rfd::MessageLevel::Error)
        .show();

    match result {
        MessageDialogResult::Yes => {}          // Stubbed, we don't need them in main but rustc complains if we exclude them
        MessageDialogResult::No => {}           // ^
        MessageDialogResult::Custom(_) => {}    // ^

        MessageDialogResult::Ok => {
            std::process::exit(1);
        }

        MessageDialogResult::Cancel => {
            std::process::exit(1);
        }
    }
}

// Fun fact: this is much easier because we know all this data will exist
// it's deterministic. But the mod search is not deterministic, oh well.
pub fn check_internal_lua_health() -> bool {
    let internal_path = "assets/scripts";
    let internal_files = vec![
        "init.lua"
    ];

    if !std::path::Path::new(internal_path).exists() {
        return false;
    }

    for file in internal_files {
        let full_path = format!("{}/{}", internal_path, file);
        if !std::path::Path::new(&full_path).exists() {
            return false;
        }
    }

    true
}

pub fn check_for_external_mods() -> bool {
    let external_path= std::path::Path::new("mods/");

    if !external_path.exists() || !external_path.is_dir() {
        return false;
    }

    let mut found_any = false;

    // This giant chunk of code looks at the dir, checks for subdirs, then
    // checks each subdir for an 'init.lua' and a 'manifest.json' to determine
    // if its a valid mod. Right now it just checks that the files exist.
    // At a later date, proper JSON parsing and Lua sandboxing will be added.
    if let Ok(entries) = fs::read_dir(external_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            // Classic path shenanigans -- but it works!
            if path.is_dir() {
                let init_file = path.join("init.lua");
                let manifest = path.join("manifest.json");

                if init_file.is_file() && manifest.is_file() {
                    if let Some(mod_name) = path.file_name().and_then(|n| n.to_str()) {
                        println!("Found {} in mods directory.", mod_name);
                    }
                    found_any = true;
                }
            }
        }
    }

    found_any
}