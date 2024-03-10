# Bevy Experiments

Contains a set of individual plugins which each represent an experiment in learning rust and the bevy game engine.

## Experiments

- triangle: Creates a mesh with a single triangle and renders it.
- shapes: Sample code which creates the six main built in shapes.
- physics blocks: Uses xpbd physics engine to simulate a block hittina a wall.
- model: Loads and renders a 3d model with shadows, environment map and animated camera.

## Notes

Not sure instancing is supported natively yet in Bevy:
https://github.com/bevyengine/bevy/issues/89
https://bevyengine.org/examples-webgpu/Shaders/shader-instancing/
https://docs.rs/bevy-aabb-instancing/latest/bevy_aabb_instancing/