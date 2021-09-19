// Taken directly from imgui-rs examples at
//
//   https://github.com/Gekkio/imgui-rs/tree/master/imgui-examples/examples/support
//
// Not my code. Originally by Joonas Javanainen and the ImGUI-rs contributors
use clipboard::{ClipboardContext, ClipboardProvider};
use imgui::ClipboardBackend;

pub struct ClipboardSupport(ClipboardContext);

pub fn init() -> Option<ClipboardSupport> {
    ClipboardContext::new().ok().map(ClipboardSupport)
}

impl ClipboardBackend for ClipboardSupport {
    fn get(&mut self) -> Option<String> {
        self.0.get_contents().ok().map(|text| text.into())
    }
    fn set(&mut self, text: &str) {
        let _ = self.0.set_contents(text.to_owned());
    }
}
