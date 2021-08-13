use gladis::Gladis;
use gtk::prelude::*;
use std::rc::Rc;

use crate::app::components::{
    screen_add_css_provider, Component, EventListener, MiniPlayer, Playlist,
};
use crate::app::{state::PlaybackEvent, AppEvent};
use crate::app::{AppAction, Worker};

use super::NowPlayingModel;

#[derive(Clone, Gladis)]
struct NowPlayingWidget {
    root: gtk::Widget,
    listbox: gtk::ListBox,
    mini_player: gtk::Button,
}

impl NowPlayingWidget {
    fn new() -> Self {
        screen_add_css_provider(resource!("/components/now_playing.css"));
        Self::from_resource(resource!("/components/now_playing.ui")).unwrap()
    }
}

pub struct NowPlaying {
    widget: NowPlayingWidget,
    model: Rc<NowPlayingModel>,
    worker: Worker,
    mini_player: Option<MiniPlayer>,
    children: Vec<Box<dyn EventListener>>,
}

impl NowPlaying {
    pub fn new(model: Rc<NowPlayingModel>, worker: Worker) -> Self {
        let widget = NowPlayingWidget::new();
        let playlist = Playlist::new(widget.listbox.clone(), model.clone());

        widget
            .mini_player
            .connect_clicked(clone!(@weak model => move |_| {
                model.dispatcher.dispatch(AppAction::ViewMiniPlayer);
            }));

        Self {
            widget,
            model,
            worker,
            mini_player: None,
            children: vec![Box::new(playlist)],
        }
    }

    fn update_mini_player(&self) {
        self.mini_player
            .as_ref()
            .map(|player| player.update_current_info());
    }

    fn update_mini_player_events(&mut self, event: &AppEvent) {
        self.mini_player
            .as_mut()
            .map(|player| player.update_controls(event));
    }
}

impl Component for NowPlaying {
    fn get_root_widget(&self) -> &gtk::Widget {
        &self.widget.root
    }

    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn EventListener>>> {
        Some(&mut self.children)
    }
}

impl EventListener for NowPlaying {
    fn on_event(&mut self, event: &AppEvent) {
        match event {
            AppEvent::PlaybackEvent(pevent) => {
                match pevent {
                    PlaybackEvent::TrackChanged(_) => {
                        self.model.load_more_if_needed();
                        self.update_mini_player();
                    }
                    PlaybackEvent::PlaylistChanged | PlaybackEvent::PlaybackStopped => {
                        self.update_mini_player()
                    }
                    _ => (),
                }
                self.update_mini_player_events(event);
            }
            AppEvent::MiniPlayerShown => {
                self.mini_player = Some(MiniPlayer::new(self.model.clone(), self.worker.clone()));
                self.update_mini_player();
                self.update_mini_player_events(event);
            }
            AppEvent::MiniPlayerDestroy => {
                self.mini_player.take();
            }
            _ => (),
        }
        self.broadcast_event(event);
    }
}
