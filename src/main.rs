mod asset_loader;
mod camera;
mod cone;
mod cube;
mod cylinder;
mod environment;
mod staff;

use bevy::prelude::*;

use self::asset_loader::AssetLoaderPlugin;
use self::camera::CameraPlugin;
use self::environment::EnvironmentPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#canvas".into()),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(CameraPlugin)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(AssetLoaderPlugin)
        .run();
}
