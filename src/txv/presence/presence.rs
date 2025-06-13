use discord_rich_presence::{activity::{self, Assets}, DiscordIpc, DiscordIpcClient};
use std::{thread, time::Duration};

/// Spawns a thread for updating Discord Rich Presence depending on 
/// the current file, which is passed into the thread by the initial 
/// call to the editor.
pub fn presence(filename: String, extension: String) {
    // A vector containing all of the allowed extensions.
    let allowed_extensions: Vec<&str> = vec!("c", "cpp", "dart", "rs");

    // Creates the full path to the file icon depending on file extension.
    let small_image: String = 
    match allowed_extensions.iter().position(|&s| s == extension) {
        Some(_) => { format!("txv_{}_icon", &extension) }
        None => { String::from("online") }
    };

    // Changes the small text message depending on file extension.
    let small_text: String = format!("Editing a .{} file", &extension);
                
    // Creates a thread that loops every 10 seconds,
    // sending an API call to Discord's RP handler.
    thread::spawn(move || {
        loop {
            // Connects to the client and creates RP information.
            let mut client = DiscordIpcClient::new("1352769914976342077").unwrap();
            let display: Assets = Assets::new()
                .large_image("txv_logo")
                .large_text("~ :txv")
                .small_image(&small_image)
                .small_text(&small_text);

            let mut detail = String::from("Editing ");
            detail.push_str(&filename);

            // Attempts to connect with the client.
            match client.connect() {
                Ok(()) => {
                    let payload = activity::Activity::new()
                        .details("~ :txv 1.1.0").assets(display).state(&detail);
                    
                    match client.set_activity(payload) {
                        Ok(()) => {}, 
                        Err(_err) => { println!("Activity error."); }
                    }
                },
                Err(_err) => { println!("Connection error."); }
            }

            thread::sleep(Duration::from_secs(3600));
        }
    });
}