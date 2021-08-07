use gladis::Gladis;
use gtk::prelude::*;
use std::rc::Rc;

use crate::app::BrowserEvent;
use crate::app::components::{screen_add_css_provider, Component, EventListener};
use crate::app::AppEvent;

use super::AlbumInfoModel;

#[derive(Clone, Gladis)]
struct AlbumInfoWidget {
    root: gtk::Widget,
    listbox: gtk::ListBox,
}

impl AlbumInfoWidget {
    fn new() -> Self {
        screen_add_css_provider(resource!("/components/album_info.css"));
        Self::from_resource(resource!("/components/album_info.ui")).unwrap()
    }
}

pub struct Info {
    widget: AlbumInfoWidget,
    model: Rc<AlbumInfoModel>,
}

impl Info {
    pub fn new(model: Rc<AlbumInfoModel>) -> Self {
        let widget = AlbumInfoWidget::new();
        model.load_album_info_detail();
        Self { widget, model }
    }

    fn update_info(&mut self) {
        if let Some(info) = self.model.get_album_info() {
            println!("{:#?}", &*info);
        }
    }
}

impl Component for Info {
    fn get_root_widget(&self) -> &gtk::Widget {
        &self.widget.root
    }

    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn EventListener>>> {
        None
    }
}

impl EventListener for Info {
    fn on_event(&mut self, event: &AppEvent) {
        if let AppEvent::BrowserEvent(BrowserEvent::AlbumInfoUpdated) = event {
            self.update_info();
        }
        self.broadcast_event(event);
    }
}