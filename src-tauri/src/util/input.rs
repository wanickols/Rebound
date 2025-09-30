use crate::game::gamemanager::GameManager;

pub struct InputHandler;

impl InputHandler {
 pub fn handle_input(key: String, pressed: bool, gm: &mut GameManager) {
        match key.as_str() {
            "ArrowUp" => gm.update(pressed, false, false, false),
            "ArrowDown" => gm.update(false, pressed, false, false),
            "ArrowLeft" => gm.update(false, false, pressed, false),
            "ArrowRight" => gm.update(false, false, false, pressed),
            _ => {}
        }
    }
}