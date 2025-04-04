mod txv { include!("txv.rs"); }

use crate::txv::editor::Editor;
use crate::txv::presence;

pub fn main() {
    // Launches a new editor program.
    let mut editor = Editor::default();

    // The arguments passed into the command line.
    let args: Vec<String> = std::env::args().collect();

    // If a file exists at the specified location, load it into the editor.
    if let Some(file) = args.get(1) {
        editor.load(file).unwrap();
    }

    // Cuts off the filename to show the file name for rich presence.
    let mut filename: String = String::from(args.get(1).unwrap());
    let file_truncated: String;
    match filename.rfind('/') {
        Some(ind) => { file_truncated = filename[(ind + 1)..].to_string(); },
        None => { file_truncated = filename; }
    }

    // Cuts off the file extension for highlighting certain syntax.
    let extension: String;
    filename = String::from(file_truncated.clone());
    match filename.rfind('.') {
        Some(ind) => { extension = filename[(ind + 1)..].to_string(); }
        None => { extension = filename; }
    }

    // Displays Rich Presence on Discord if connected.
    presence::presence(file_truncated);

    // Runs the editor program.
    editor.set_extension(extension);
    editor.run();
}