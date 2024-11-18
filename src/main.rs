mod editor { include!("editor.rs");}
use editor::Editor;

pub fn main() {
    Editor::default().run();
}