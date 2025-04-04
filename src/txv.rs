/// Main editor handler.
pub mod editor { include!("txv/editor.rs");}

pub mod buffer { include!("txv/text/buffer.rs"); }
pub mod view { include!("txv/text/view.rs"); }

pub mod colors { include!("txv/visuals/colors.rs"); }
pub mod highlighter { include!("txv/visuals/highlighter.rs"); }
pub mod terminal { include!("txv/visuals/terminal.rs"); }

pub mod hls { include!("txv/visuals/hls/hls.rs"); }

pub mod presence { include!("txv/presence/presence.rs"); }