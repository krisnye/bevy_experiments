use bevy::app::PluginGroupBuilder;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub struct FPSDisplayPluginGroup;
impl PluginGroup for FPSDisplayPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FrameTimeDiagnosticsPlugin)
            .add(LogDiagnosticsPlugin::default())
        // Any plugin can register diagnostics. Uncomment this to add an entity count diagnostics:
        // bevy::diagnostic::EntityCountDiagnosticsPlugin::default(),
        // Uncomment this to add an asset count diagnostics:
        // bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<Texture>::default(),
        // Uncomment this to add system info diagnostics:
        // bevy::diagnostic::SystemInformationDiagnosticsPlugin::default()
    }
}
