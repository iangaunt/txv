use discord_rich_presence::{activity::{self, Assets}, DiscordIpc, DiscordIpcClient};
use std::thread;
use std::time::Duration;

pub fn presence(filename: String) {
    thread::spawn(move || {
        loop {
            let mut client = DiscordIpcClient::new("1352769914976342077").unwrap();
            let display: Assets = Assets::new()
                .large_image("txv_logo")
                .large_text("~ :txv")
                .small_image("online")
                .small_text("Editing a file");

            let mut detail = String::from("Editing ");
            detail.push_str(&filename);

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

            thread::sleep(Duration::from_secs(10000));
        }
    });
}