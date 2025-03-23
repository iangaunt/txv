mod txv { include!("txv.rs"); }

use crate::txv::editor::Editor;
use crate::txv::presence;

pub fn main() {
    let mut editor = Editor::default();
    let args: Vec<String> = std::env::args().collect();

    if let Some(file) = args.get(1) {
        editor.load(file).unwrap();
    }

    let filename: String = String::from(args.get(1).unwrap());
    let file_truncated: String;
    match filename.rfind('/') {
        Some(ind) => { file_truncated = filename[(ind + 1)..].to_string(); },
        None => { file_truncated = filename; }
    }

    presence::presence(file_truncated);
    editor.run();
}