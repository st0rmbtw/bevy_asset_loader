#![allow(dead_code, unused_imports)]

use bevy::app::AppExit;
use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[cfg(all(
    feature = "2d",
    feature = "3d",
    feature = "standard_dynamic_assets",
    not(feature = "progress_tracking"),
))]
#[test]
fn main() {
    App::new()
        .add_state::<MyStates>()
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin::default())
        .add_plugin(AudioPlugin::default())
        .insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)))
        .add_loading_state(
            LoadingState::new(MyStates::SplashAssetLoading).continue_to_state(MyStates::Splash),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            MyStates::SplashAssetLoading,
            "full_dynamic_collection.assets.ron",
        )
        .add_system(splash_countdown.in_set(OnUpdate(MyStates::Splash)))
        .add_loading_state(
            LoadingState::new(MyStates::MainMenuAssetLoading).continue_to_state(MyStates::MainMenu),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            MyStates::MainMenuAssetLoading,
            "full_dynamic_collection.assets.ron",
        )
        .add_collection_to_loading_state::<_, MainMenuAssets>(MyStates::MainMenuAssetLoading)
        .add_system(timeout)
        .add_system(quit.in_set(OnUpdate(MyStates::MainMenu)))
        .run();
}

#[derive(AssetCollection, Resource)]
struct SplashAssets {
    #[asset(key = "single_file")]
    another_file: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
struct MainMenuAssets {
    #[asset(key = "single_file")]
    single_file: Handle<AudioSource>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    SplashAssetLoading,
    Splash,
    MainMenuAssetLoading,
    MainMenu,
}

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_countdown(
    mut game_state: ResMut<NextState<MyStates>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(MyStates::MainMenuAssetLoading);
    }
}

fn timeout(time: Res<Time>) {
    if time.elapsed_seconds_f64() > 30. {
        panic!("The app did not finish in 30 seconds");
    }
}

fn quit(mut exit: EventWriter<AppExit>) {
    info!("Everything fine, quitting the app");
    exit.send(AppExit);
}
