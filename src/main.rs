// External crates
use raylib::prelude::*;
use rfd::MessageDialogResult;
use std::fs;

// Local modules
mod player;

fn raise_error(title: &str, message: &str) {
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
fn check_internal_lua_health() -> bool {
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

fn check_for_external_mods() -> bool {
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

fn main() {
    // Initalize Raylib before init of other subsystems
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Crystal Caverns")
        .build();

    rl.trace_log(TraceLogLevel::LOG_INFO, "Raylib initalized, moving to subsystems.");
    rl.set_target_fps(60);

    // Subsystems init (player, items, NPCs, misc...)
    let mut hero = player::Player::new(Vector2::new(400.0, 300.0), 120.0, 32.0, Color::RED);

    // if !check_internal_lua_health() {
    //     raise_error("Lua Internal Fatal Error", "LuaInternal files are misssing or damaged and must be reinstalled. Please refer to the README for assistance.");
    // }

    if !check_for_external_mods() {
        rl.trace_log(TraceLogLevel::LOG_WARNING, "Mods will be disabled for this session.");
    }
    
    rl.trace_log(TraceLogLevel::LOG_INFO, "Subsystems ready, starting rendering.");

    // Game loop starts here
    while !rl.window_should_close() {
        // Updating everything here
        let dt = rl.get_frame_time();

        hero.update(&rl, dt);

        // Drawing result to buffer
        /************************************************************************************/
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("CrystalCaverns Rust Technical Demo", 0, 0, 20, Color::WHITE);
        d.draw_fps(0, 20);

        hero.draw(&mut d);

        /************************************************************************************/
        // Once here, restart the loop and go forevermore
    }
}