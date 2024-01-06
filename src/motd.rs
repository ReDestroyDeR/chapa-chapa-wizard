use bevy::app::Plugin;
use bevy::asset::AssetServer;
use bevy::asset::{Assets, Handle};
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use rand::Rng;

pub struct MotdPlugin;

impl Plugin for MotdPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<Messages>::new(&["motd.json"]))
            .add_state::<MotdStates>()
            .add_loading_state(
                LoadingState::new(MotdStates::LoadingMotd).continue_to_state(MotdStates::Ready),
            )
            .add_collection_to_loading_state::<_, MessagesAsset>(MotdStates::LoadingMotd)
            .add_systems(OnEnter(MotdStates::Ready), add_message_of_the_day);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MotdStates {
    #[default]
    LoadingMotd,
    Ready,
}

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "53114379-7223-4335-a568-2c1d8b56b522"]
struct Messages {
    window_messages: Vec<String>,
}

impl Messages {
    fn get_random(&self) -> Option<&str> {
        if self.window_messages.len() == 0 {
            None
        } else {
            self.window_messages
                .get(rand::thread_rng().gen_range(0..self.window_messages.len()))
                .map(|s| s.as_str())
        }
    }
}

#[derive(AssetCollection, Resource)]
struct MessagesAsset {
    #[asset(path = "messages.motd.json")]
    messages: Handle<Messages>,
}

const FAILED_TO_GET_MESSAGE: &str = "I failed miserably";

fn add_message_of_the_day(
    messages_res: Res<MessagesAsset>,
    messages_assets: Res<Assets<Messages>>,
    mut windows_query: Query<&mut Window>,
) {
    let message: &str = messages_assets
        .get(&messages_res.messages)
        .and_then(|m| m.get_random())
        .unwrap_or_else(|| FAILED_TO_GET_MESSAGE);

    println!("size {}", windows_query.iter().len());
    for mut window in windows_query.iter_mut() {
        window.title.push_str(&format!(": {message}"));
    }
}
