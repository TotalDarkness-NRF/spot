use gladis::Gladis;
use gtk::prelude::*;
use std::rc::Rc;

use crate::app::components::{screen_add_css_provider, Component, EventListener};
use crate::app::{state::PlaybackEvent, AppEvent};

use super::SettingsModel;

#[derive(Clone, Gladis)]
struct SettingsWidget {
    root: gtk::Widget,
    listbox: gtk::ListBox,
}

impl SettingsWidget {
    fn new() -> Self {
        screen_add_css_provider(resource!("/components/settings.css"));
        Self::from_resource(resource!("/components/settings.ui")).unwrap()
    }
}

pub struct Settings {
    widget: SettingsWidget,
    model: Rc<SettingsModel>,
    children: Vec<Box<dyn EventListener>>,
}

impl Settings {
    pub fn new(model: Rc<SettingsModel>) -> Self {
        let widget = SettingsWidget::new();
        Self {
            widget,
            model,
            children: vec![],
        }
    }
}

impl Component for Settings {
    fn get_root_widget(&self) -> &gtk::Widget {
        &self.widget.root
    }

    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn EventListener>>> {
        Some(&mut self.children)
    }
}

impl EventListener for Settings {
    fn on_event(&mut self, event: &AppEvent) {
        if let AppEvent::PlaybackEvent(PlaybackEvent::TrackChanged(_)) = event {
            
        }
        self.broadcast_event(event);
    }
}
