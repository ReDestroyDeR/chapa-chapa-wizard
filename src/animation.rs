use anyhow::{anyhow, Result};
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::json::JsonAssetPlugin;
use dashmap::DashMap;
use serde::Deserialize;

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AnimationLoadingStates>()
            .add_plugins(JsonAssetPlugin::<AnimationBundle>::new(&[
                "animations.json",
            ]))
            .add_loading_state(
                LoadingState::new(AnimationLoadingStates::LoadingSprites)
                    .continue_to_state(AnimationLoadingStates::Ready),
            )
            .add_systems(Update, animate_sprite);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AnimationLoadingStates {
    #[default]
    LoadingSprites,
    Ready,
}

#[derive(Default, Component, Clone, Debug)]
pub struct Animations(DashMap<String, SpriteAnimation>);

impl Animations {
    pub fn get(&self, animation: &str) -> SpriteAnimation {
        self.0
            .get(animation)
            .expect(&format!("No animation {animation} found"))
            .value()
            .clone()
    }
}

#[derive(Bundle, Default, TypePath, TypeUuid, Clone, Deserialize)]
#[uuid = "185a795e-a515-43ac-89f1-ddf1bfdfa667"]
#[serde(try_from="AnimationConfig")]
pub struct AnimationBundle {
    default: CurrentAnimation,
    animations: Animations,
}

impl TryFrom<AnimationConfig> for AnimationBundle {
    type Error = anyhow::Error;

    fn try_from(cfg: AnimationConfig) -> Result<Self, Self::Error> {
        Self::new(&cfg)
    }
}

impl AnimationBundle {
    fn new(cfg: &AnimationConfig) -> Result<Self> {
        if let AnimationConfig::Animated {
            default,
            animations,
        } = cfg
        {
            if animations.is_empty() {
                Err(anyhow!(
                    "Can't create animation controller. Animations are empty"
                ))
            } else if animations.get(default).is_none() {
                Err(anyhow!(
                    "No default animation \"{default}\" found in animations:\n{animations:#?}"
                ))
            } else {
                let current = animations.get(default).unwrap().clone();
                Ok(Self {
                    default: CurrentAnimation(current),
                    animations: Animations(animations.clone()),
                })
            }
        } else {
            Ok(self::default())
        }
    }
}

#[derive(Deserialize)]
enum AnimationConfig {
    NoAnimation,
    Animated {
        default: String,
        animations: DashMap<String, SpriteAnimation>,
    },
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct CurrentAnimation(SpriteAnimation);

impl Default for CurrentAnimation {
    fn default() -> Self {
        Self(EMPTY)
    }
}

impl CurrentAnimation {
    pub fn change(
        &mut self,
        to: &SpriteAnimation,
        sprite: &mut TextureAtlasSprite,
        timer: &mut AnimationTimer,
    ) {
        if self.0.ne(to) {
            self.0 = to.clone();
            sprite.index = to.first;
            *timer = AnimationTimer::from(to.speed);
        }
    }
}

const EMPTY: SpriteAnimation = SpriteAnimation {
    first: 0,
    last: 0,
    speed: 0.0,
};

#[derive(Deserialize, Debug, Clone)]
pub struct SpriteAnimation {
    first: usize,
    last: usize,
    speed: f32,
}

impl PartialEq for SpriteAnimation {
    fn eq(&self, other: &Self) -> bool {
        other.first == self.first && other.last == self.last
    }
}

#[derive(Default, Deref, DerefMut, Component, Debug, Clone)]
pub struct AnimationTimer(pub Timer);

impl From<f32> for AnimationTimer {
    fn from(value: f32) -> Self {
        Self(Timer::from_seconds(value / 1000., TimerMode::Repeating))
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &CurrentAnimation,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (current, mut timer, mut sprite) in &mut query {
        let SpriteAnimation {
            first,
            last,
            speed: _,
        } = current.0;
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == last {
                first
            } else {
                sprite.index + 1
            };
        }
    }
}
