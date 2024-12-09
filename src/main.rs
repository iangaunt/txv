mod buffer { include!("editor/text/buffer.rs"); }
mod editor { include!("editor/editor.rs");}
mod highlighter { include!("editor/visuals/highlighter.rs"); }
mod terminal { include!("editor/visuals/terminal.rs"); }
mod view { include!("editor/text/view.rs"); }

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