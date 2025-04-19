// External crates
use raylib::prelude::*;


// Local modules
mod player;
mod utils;

fn main() {
    // Initalize Raylib before init of other subsystems
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Crystal Caverns")
        .build();

    rl.trace_log(TraceLogLevel::LOG_INFO, "Raylib initalized, moving to subsystems.");
    rl.set_target_fps(90);

    // Subsystems init (player, items, NPCs, misc...)
    let mut hero = player::Player::new(Vector2::new(400.0, 300.0), 120.0, 32.0, Color::RED, ("no").to_string());

    if utils::check_internal_lua_health() {
        utils::raise_error("Lua Internal Fatal Error", "LuaInternal files are misssing or damaged and must be reinstalled. Please refer to the README for assistance.");
    }

    if !utils::check_for_external_mods() {
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