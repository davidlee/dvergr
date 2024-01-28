use bevy::prelude::Component;

#[derive(Component, Debug)]
pub(crate) struct ChunkMarker;

#[derive(Component, Debug)]
pub(crate) struct BoardMarker;

#[derive(Component, Debug)]
pub(crate) struct MapMarker;

#[derive(Component, Debug)]
pub(crate) struct TorchMarker;

#[derive(Component, Debug)]
pub(crate) struct TorchSecondaryLightMarker;

#[derive(Component, Debug)]
pub(crate) struct CameraMarker;
