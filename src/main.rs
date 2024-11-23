mod buffer { include!("editor/buffer.rs"); }
mod editor { include!("editor/editor.rs");}
mod terminal { include!("editor/terminal.rs"); }
mod view { include!("editor/view.rs"); }

use editor::Editor;

// cd ./target/debug :: ./rust-projects.exe ./text.txt

pub fn main() {
    let mut editor = Editor::default();

    let args: Vec<String> = std::env::args().collect();
    if let Some(file) = args.get(1) {
        editor.load(file).unwrap();
    }

    editor.run();
}