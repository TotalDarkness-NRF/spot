use std::rc::Rc;

use crate::app::components::{EventListener, NowPlayingModel};
use crate::app::loader::ImageLoader;
use crate::app::{AppAction, AppEvent, Worker};
use gettextrs::gettext;
use gtk::prelude::*;

pub struct MiniPlayer {
    model: Rc<NowPlayingModel>,
    worker: Worker,
    controls: Box<dyn EventListener>,
    current_song_info: gtk::Label,
    playing_image: gtk::Image,
}

impl MiniPlayer {
    pub fn new(model: Rc<NowPlayingModel>, worker: Worker) -> Self {
        let builder = gtk::Builder::from_resource(resource!("/components/mini_player.ui"));

        let player: libhandy::Window = builder.object("mini_player").unwrap();
        let controls: Box<dyn EventListener> = crate::app::App::make_playback_control(
            &builder,
            model.app_model.clone(),
            model.dispatcher.box_clone(),
        );
        let close_button: gtk::Button = builder.object("close_button").unwrap();

        player.connect_destroy(clone!(@weak model => move |_| {
            model.dispatcher.dispatch(AppAction::DestroyMiniPlayer);
            println!("Destroy de");
        }));

        player.connect_delete_event(
            clone!(@weak model => @default-return glib::signal::Inhibit(true), move |_, _| {
                model.dispatcher.dispatch(AppAction::DestroyMiniPlayer);
                println!("Destroy delete");
                glib::signal::Inhibit(false)
            }),
        );
        player.connect_key_press_event(|player, event| {
            if let gdk::keys::constants::Escape = event.keyval() {
                unsafe { player.destroy() };
                println!("Destroy key");
            }
            glib::signal::Inhibit(false)
        });

        close_button.connect_clicked(move |_| unsafe {
            player.destroy();
            println!("Destroy click");
        });

        let current_song_info = builder.object("current_song_info").unwrap();
        let playing_image = builder.object("playing_image").unwrap();
        
        // TODO have pin on top button

        Self {
            model,
            worker,
            controls,
            current_song_info,
            playing_image,
        }
    }

    pub fn update_current_info(&self) {
        if let Some(song) = self.model.queue().current_song() {
            let title = glib::markup_escape_text(&song.title);
            let artist = glib::markup_escape_text(&song.artists_name());
            let label = format!("<b>{}</b>\n{}", title.as_str(), artist.as_str());
            self.current_song_info.set_label(&label[..]);

            let image = self.playing_image.clone();
            if let Some(url) = song.art.clone() {
                self.worker.send_local_task(async move {
                    let loader = ImageLoader::new();
                    let result = loader.load_remote(&url, "jpg", 200, 200).await;
                    image.set_from_pixbuf(result.as_ref());
                });
            }
        } else {
            self.current_song_info
                .set_label(&gettext("No song playing"));
            self.playing_image.set_from_icon_name(
                Some("emblem-music-symbolic"),
                self.playing_image.icon_size(),
            );
        }
    }

    pub fn update_controls(&mut self, event: &AppEvent) {
        self.controls.on_event(event);
    }
}
