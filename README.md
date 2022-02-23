# Voxelcraft

Voxelcraft is voxel game with a lot of inspiration from Minecraft and its modded community. Voxelcraft does not aim to 
be a full game, intead it's more of a platform that allows for "mods" to customize the experience.

# FAQ

### Is this game windows/macos/linux only?
No the goal is to make sure everything compiles down to as many platforms as possible. The game is backed by crates 
such as winit and wgpu that takes care of most of the platform specific dependencies.

### Why rust

Rust is blazing fast, but still friendly to use. It also doesn't suffer from a garbage collector, and the sudden fps 
drops that comes with that.

The goal is to in difference from Minecraft that makes mod "stitching" at runtime, to instead deffer as much as possible 
to compile time. 

### Is there a max height in this game

No, the chunks are 3 dimensional, so that means infinite height as well, or the max number a i64 integer can hold 
(9223372036854775807).