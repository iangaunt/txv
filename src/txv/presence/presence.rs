use discord_rich_presence::{activity::{self, Assets}, DiscordIpc, DiscordIpcClient};
use std::{thread, time::Duration};

/// Spawns a thread for updating Discord Rich Presence depending on 
/// the current file, which is passed into the thread by the initial 
/// call to the editor.
pub fn presence(filename: String) {
    // Creates a thread that loops every 10 seconds,
    // sending an API call to Discord's RP handler.
    thread::spawn(move || {
        loop {
            // Connects to the client and creates RP information.
            let mut client = DiscordIpcClient::new("1352769914976342077").unwrap();
            let display: Assets = Assets::new()
                .large_image("txv_logo")
                .large_text("~ :txv")
                .small_image("online")
                .small_text("Editing a file");

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