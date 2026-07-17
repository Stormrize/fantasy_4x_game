use std::net::TcpStream;
use std::net::TcpListener;
use bevy::prelude::{ResMut, Resource};

#[derive(Resource)]
pub struct Network {
    pub stream: TcpStream,
}

fn network_receive(
    mut network: ResMut<Network>,
) {

}