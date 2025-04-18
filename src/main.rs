use raylib::prelude::*;
use rfd::MessageDialogResult;

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
            eprintln!("");
            std::process::exit(1);
        }
    }
}

fn check_internal_lua_health() -> bool {
    return false;
}

fn check_for_external_mods() -> bool {
    return false;
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
    if !check_internal_lua_health() {
        raise_error("Lua Internal Fatal Error", "LuaInternal files are misssing or damaged and must be reinstalled. Please refer to Steam Game Validation for assistance.");
    }

    if !check_for_external_mods() {
        rl.trace_log(TraceLogLevel::LOG_INFO, "Mods will be disabled for this session.");
    }
    
    rl.trace_log(TraceLogLevel::LOG_INFO, "Subsystems ready, starting rendering.");

    // Game loop starts here
    while !rl.window_should_close() {
        // Updating everything here
        let dt = rl.get_frame_time();

        // Drawing result to buffer
        /************************************************************************************/
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("CrystalCaverns Rust Technical Demo", 0, 0, 20, Color::WHITE);
        d.draw_fps(0, 20);

        /************************************************************************************/
        // Once here, restart the loop and go forevermore
    }
}