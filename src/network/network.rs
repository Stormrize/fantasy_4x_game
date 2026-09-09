use bevy::prelude::{ResMut, Resource};
use std::net::TcpStream;

#[derive(Resource)]
pub struct Network {
    pub stream: TcpStream,
}

fn network_receive(mut network: ResMut<Network>) {}
