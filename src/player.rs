use raylib::prelude::*;

pub struct Player {
    position: Vector2,
    speed: f32,
    size: f32,
    color: Color,
    asset_path: String,
}

impl Player {
    pub fn new(position: Vector2, speed: f32, size: f32, color: Color, asset_path: String) -> Self {
        Self {
            position,
            speed,
            size,
            color,
            asset_path,
        }
    }

    // TODO: Refactor this mess to be a bit less awful!
    pub fn update(&mut self, rl: &RaylibHandle, dt: f32) {
        // Movement controls
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
            let mut sprint_modifier: f32 = 1.0;
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) {
                sprint_modifier = 1.66;
            }
            self.position.x += (self.speed * sprint_modifier) * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
            let mut sprint_modifier: f32 = 1.0;
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) {
                sprint_modifier = 1.66;
            }
            self.position.x -= (self.speed * sprint_modifier) * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) {
            let mut sprint_modifier: f32 = 1.0;
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) {
                sprint_modifier = 1.66;
            }
            self.position.y -= (self.speed * sprint_modifier) * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) {
            let mut sprint_modifier: f32 = 1.0;
            if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) {
                sprint_modifier = 1.66;
            }
            self.position.y += (self.speed * sprint_modifier) * dt;
        }

        // Note: the ability to modify keybinds will be handled later in development,
        //       likely once the player system is more concrete.
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        if !std::path::Path::new(&self.asset_path).exists() {
            d.draw_circle_v(self.position, self.size, self.color);
        } else {
          // draw the player asset when it inevitably comes around
        }
    }
}