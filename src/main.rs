mod txv { include!("txv.rs"); }

use crate::txv::editor::Editor;
use crate::txv::presence;

pub fn main() {
    // Launches a new editor program.
    let mut editor = Editor::default();
    let args: Vec<String> = std::env::args().collect();

    // If a file exists at the specified location, load it into the editor.
    if let Some(file) = args.get(1) {
        // Loads the file at the file location.
        if editor.load(file).is_err() { return; }

        // Cuts off the filename to show the file name for rich presence.
        let mut filename: String = String::from(args.get(1).unwrap());
        let file_truncated: String = match filename.rfind('/') {
            Some(ind) => { filename[(ind + 1)..].to_string() },
            None => { filename }
        };

        // Cuts off the file extension for highlighting certain syntax.
        filename = file_truncated.clone();
        let extension: String = match filename.rfind('.') {
            Some(ind) => { filename[(ind + 1)..].to_string() }
            None => { filename }
        };

        // Displays Rich Presence on Discord if connected.
        presence::presence(file_truncated, extension.clone());

        // Runs the editor program.
        editor.set_extension(extension);
    }

    editor.run();
}