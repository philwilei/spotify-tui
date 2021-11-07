use super::{super::app::App, common_key_events};
use crate::{
  app::{ActiveBlock, AlbumTableContext, RouteId},
  event::Key,
  network::IoEvent,
};

pub fn handler(key: Key, app: &mut App) {
  match key {
    k if common_key_events::left_event(k) => common_key_events::handle_left_event(app),
    k if common_key_events::down_event(k) => {
      if let Some(new_releases) = &mut app.new_releases.get_results(None) {
        let next_index =
          common_key_events::on_down_press_handler(&new_releases.items, Some(app.new_releases_index));
        app.new_releases_index = next_index;
      }
    }
    k if common_key_events::up_event(k) => {
      if let Some(new_releases) = &mut app.new_releases.get_results(None) {
        let next_index =
          common_key_events::on_up_press_handler(&new_releases.items, Some(app.new_releases_index));
        app.new_releases_index = next_index;
      }
    }
    k if common_key_events::high_event(k) => {
      if let Some(_new_releases) = app.new_releases.get_results(None) {
        let next_index = common_key_events::on_high_press_handler();
        app.new_releases_index = next_index;
      }
    }
    k if common_key_events::middle_event(k) => {
      if let Some(new_releases) = app.new_releases.get_results(None) {
        let next_index = common_key_events::on_middle_press_handler(&new_releases.items);
        app.new_releases_index = next_index;
      }
    }
    k if common_key_events::low_event(k) => {
      if let Some(new_releases) = app.new_releases.get_results(None) {
        let next_index = common_key_events::on_low_press_handler(&new_releases.items);
        app.new_releases_index = next_index;
      }
    }
    Key::Enter => {
      if let Some(new_releases) = app.new_releases.get_results(None) {
        if let Some(selected_release) = new_releases.items.get(app.new_releases_index).cloned() {
          // Assigns the track list for the selected album to app.selected_album_simplified
          app.dispatch(IoEvent::GetAlbumTracks(Box::new(selected_release)));
          app.album_table_context = AlbumTableContext::Simplified;
          app.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
        }
      }
    }
    _ => {}
  }
}
