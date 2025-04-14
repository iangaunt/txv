use discord_rich_presence::{activity::{self, Assets}, DiscordIpc, DiscordIpcClient};
use std::{thread, time::Duration};

/// Spawns a thread for updating Discord Rich Presence depending on 
/// the current file, which is passed into the thread by the initial 
/// call to the editor.
pub fn presence(filename: String, extension: String) {
    // Creates the full path to the file icon depending on file extension.
    let mut small_image: String = String::from("online");
    if extension == "c" || extension == "cpp" || extension == "dart" || extension == "rs" {
        small_image = String::from("txv_");
        small_image.push_str(&extension);
        small_image += "_icon";
    }

    // Changes the small text message depending on file extension.
    let mut small_text: String = String::from("Editing a .");
    small_text.push_str(&extension);
    small_text += " file";
                
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
                        .details("~ :txv 1.0.0").assets(display).state(&detail);
                    match client.set_activity(payload) {
                        Ok(()) => {}, 
                        Err(_err) => { println!("activity error"); }
                    }
                },
                Err(_err) => { println!("connection error"); }
            }

            thread::sleep(Duration::from_secs(86400));
        }
    });
}