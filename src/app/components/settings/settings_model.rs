use gio::prelude::*;
use gio::SimpleActionGroup;
use std::cell::Ref;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use crate::app::components::{labels, PlaylistModel, SelectionTool, SelectionToolsModel};
use crate::app::models::SongDescription;
use crate::app::models::SongModel;
use crate::app::state::{
    PlaybackAction, PlaybackEvent, PlaybackState, PlaylistSource, SelectionAction,
    SelectionContext, SelectionState,
};
use crate::app::{ActionDispatcher, AppAction, AppEvent, AppModel, AppState, ListDiff};
use crate::{api::SpotifyApiClient, app::components::SimpleSelectionTool};

pub struct SettingsModel {
    app_model: Rc<AppModel>,
    dispatcher: Box<dyn ActionDispatcher>,
}

impl SettingsModel {
    pub fn new(app_model: Rc<AppModel>, dispatcher: Box<dyn ActionDispatcher>) -> Self {
        Self {
            app_model,
            dispatcher,
        }
    }

    fn state(&self) -> Ref<'_, AppState> {
        self.app_model.get_state()
    }
}

impl SelectionToolsModel for SettingsModel {
    fn dispatcher(&self) -> Box<dyn ActionDispatcher> {
        self.dispatcher.box_clone()
    }

    fn spotify_client(&self) -> Arc<dyn SpotifyApiClient + Send + Sync> {
        self.app_model.get_spotify()
    }

    fn selection(&self) -> Option<Box<dyn Deref<Target = SelectionState> + '_>> {
        let selection = self
            .app_model
            .map_state_opt(|s| Some(&s.selection))
            .filter(|s| s.context == SelectionContext::Queue)?;
        Some(Box::new(selection))
    }

    fn tools_visible(&self, _: &SelectionState) -> Vec<SelectionTool> {
        vec![
            SelectionTool::Simple(SimpleSelectionTool::SelectAll),
            SelectionTool::Simple(SimpleSelectionTool::MoveDown),
            SelectionTool::Simple(SimpleSelectionTool::MoveUp),
            SelectionTool::Simple(SimpleSelectionTool::Remove),
        ]
    }

    fn handle_tool_activated(&self, selection: &SelectionState, tool: &SelectionTool) {
        match tool {
            SelectionTool::Simple(SimpleSelectionTool::SelectAll) => {
            }
            SelectionTool::Simple(SimpleSelectionTool::Remove) => {
                self.dispatcher().dispatch(AppAction::DequeueSelection);
            }
            SelectionTool::Simple(SimpleSelectionTool::MoveDown) => {
                self.dispatcher().dispatch(AppAction::MoveDownSelection);
            }
            SelectionTool::Simple(SimpleSelectionTool::MoveUp) => {
                self.dispatcher().dispatch(AppAction::MoveUpSelection);
            }
            _ => self.default_handle_tool_activated(selection, tool),
        };
    }
}