# Bevy Experiments

Contains a set of individual plugins which each represent an experiment in learning rust and the bevy game engine.

## Experiments

- triangle: Creates a mesh with a single triangle and renders it.
- shapes: Sample code which creates the six main built in shapes.
- physics blocks: Uses xpbd physics engine to simulate a block hittina a wall.
- model: Loads and renders a 3d model with shadows, environment map and animated camera.
- voxel editor: WIP to edit voxel based shapes.

## Setting up a new Experiment / Plugin

- Copy the plugin that you want to base your new experiment off of.
- Edit the copied plugin:
  - Rename the plugin to a new name.
  - Edit the STATE constant to a new enum value.
- Edit plugins/mod.rs:
  - Add a new AppState enum entry for the new plugin using above name.
  - Optionally make the new enum entry the default by moving the #[default] above it
  - Register the new plugin in the add_systems function

## Notes

Instancing is not supported natively yet in Bevy:
https://github.com/bevyengine/bevy/issues/89
https://bevyengine.org/examples-webgpu/Shaders/shader-instancing/
https://docs.rs/bevy-aabb-instancing/latest/bevy_aabb_instancing/