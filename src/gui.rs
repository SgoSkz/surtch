use crate::results::Video;
use std::io::{stdout};

pub struct GUI {
    name: String,
    videos: Vec<Video>,
}

impl GUI {
    pub fn new() -> GUI {
        let backend = tui::backend::CrosstermBackend::new(stdout());
        let term = tui::Terminal::new(backend).unwrap();
        GUI {
            name: "HA".to_string(),
            videos: vec![],
        }
    }
}
