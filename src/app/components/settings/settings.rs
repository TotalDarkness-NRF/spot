use gio::prelude::SettingsExt;
use gladis::Gladis;
use gtk::prelude::*;
use std::rc::Rc;

use crate::app::components::{screen_add_css_provider, Component, EventListener};
use crate::app::{state::PlaybackEvent, AppEvent};
use crate::settings::SpotSettings;

use super::SettingsModel;

#[derive(Clone, Gladis)]
struct SettingsWidget {
    root: gtk::Widget,
    grid: gtk::Grid,
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
}

impl Settings {
    pub fn new(model: Rc<SettingsModel>) -> Self {
        let widget = SettingsWidget::new();
        let grid = &widget.grid;
        grid.set_halign(gtk::Align::Center);
        grid.set_valign(gtk::Align::Center);
        let label = gtk::Label::new(Some("Dark Mode"));
        let switch = gtk::Switch::new();
        switch.set_state(model.settings().prefers_dark_theme);
        switch.connect_changed_active(clone!(@weak model => @default-return (), move |switch| {
            SpotSettings::get_setting_schema().set_boolean("prefers-dark-theme", switch.state()).unwrap();
            model.settings_mut().prefers_dark_theme = switch.state();
            // TODO handle potential error
        }));

        grid.set_column_spacing(20);
        grid.attach(&label, 0, 0, 1, 1);
        grid.attach(&switch, 1, 0, 1, 1);

        let label = gtk::Label::new(Some("Maximized"));
        let switch = gtk::Switch::new();
        switch.set_state(model.settings().window.is_maximized);
        switch.connect_changed_active(clone!(@weak model => @default-return (), move |switch| {
            SpotSettings::get_setting_schema().set_boolean("window-is-maximized", switch.state()).unwrap();
            model.settings_mut().window.is_maximized = switch.state();
            println!("{}", model.settings_mut().window.is_maximized);

        }));

        grid.set_column_spacing(20);
        grid.attach(&label, 0, 1, 1, 1);
        grid.attach(&switch, 1, 1, 1, 1);
        Self { widget, model }
    }
}

impl Component for Settings {
    fn get_root_widget(&self) -> &gtk::Widget {
        &self.widget.root
    }

    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn EventListener>>> {
        None
    }
}

impl EventListener for Settings {
    fn on_event(&mut self, event: &AppEvent) {
        if let AppEvent::PlaybackEvent(PlaybackEvent::TrackChanged(_)) = event {}
        self.broadcast_event(event);
    }
}
