use bevy::{prelude::*, render::render_resource::Texture};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, load_textures)
        .run();
}

fn load_textures(
    server: Res<AssetServer>,
){
    let tileset: Handle<Image> = server.load("assets/brackeys_platformer_assets/sprites/world_tileset.png");
}
