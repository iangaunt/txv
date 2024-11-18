mod editor { include!("editor/editor.rs");}
use editor::Editor;

pub fn main() {
    Editor::default().run();
}