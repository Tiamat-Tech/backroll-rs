use backroll_transport_steam::SteamP2PManager;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_steamworks::{Client, ClientManager};
use std::ops::Deref;

fn initialize_steam_socket(client: Option<Res<Client<ClientManager>>>, mut commands: Commands) {
    if let Some(client) = client {
        commands.insert_resource(SteamP2PManager::bind(client.clone()));
    }
}

pub struct BackrollSteamPlugin;

impl Plugin for BackrollSteamPlugin {
    fn build(&self, builder: &mut App) {
        builder.add_startup_system(initialize_steam_socket);
    }
}
