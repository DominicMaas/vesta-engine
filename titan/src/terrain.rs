use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    utils::hashbrown::raw,
};
use bevy_rapier3d::parry::transformation::voxelization::Voxel;
use bracket_noise::prelude::*;

use crate::chunk::{Chunk, VoxelType, CHUNK_XZ, CHUNK_Y, WORLD_HEIGHT};

#[derive(Resource)]
pub struct Terrain {
    pub noise_func: FastNoise,
}

impl Terrain {
    pub fn new(seed: u64) -> Self {
        let mut noise_func = FastNoise::seeded(seed);
        noise_func.set_noise_type(NoiseType::SimplexFractal);
        noise_func.set_fractal_type(FractalType::FBM);
        noise_func.set_fractal_octaves(5);
        noise_func.set_fractal_gain(0.5);
        noise_func.set_fractal_lacunarity(2.0);
        noise_func.set_frequency(0.2);

        Self { noise_func }
    }

    pub fn generate_image(&self) -> Image {
        let mut data = Vec::new();

        for cx in -64..64 {
            for cz in -64..64 {
                let mut value = 0.0;

                // Determine the highest point
                for cy in 0..CHUNK_Y {
                    let y_value =
                        match self.get_block_type(Vec3::new(cx as f32, cy as f32, cz as f32)) {
                            VoxelType::Air => 0.0,
                            VoxelType::Dirt(d) => d,
                            VoxelType::Grass(d) => d,
                            VoxelType::Stone(d) => d,
                        };

                    if y_value <= 0.0 {
                        value = (cy as f32) + y_value;
                        break;
                    }
                }

                let u8v = Self::map_range((0.0, CHUNK_Y as f32), (0.0, 255.0), value) as u8;

                // Turn this range into a color (rgb as value, and fully opaque)
                data.push(u8v);
                data.push(u8v);
                data.push(u8v);
                data.push(255);
            }
        }

        Image { ..default() };

        Image::new(
            Extent3d {
                width: 128,
                height: 128,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        )
    }

    pub fn generate2(&self, world_position: Vec3) -> Chunk {
        let mut chunk = Chunk::new();

        // Load in some initial terrain
        for cx in 0..CHUNK_XZ {
            for cy in 0..CHUNK_Y {
                for cz in 0..CHUNK_XZ {
                    let c_pos = Vec3::new(cx as f32, cy as f32, cz as f32) + world_position;
                    let block_type = self.get_block_type(c_pos);

                    chunk.set_block(cx, cy, cz, block_type);
                }
            }
        }

        chunk
    }

    pub fn generate(&self, chunk: &mut Chunk, world_position: Vec3) {
        // Load in some initial terrain
        for cx in 0..CHUNK_XZ {
            for cy in 0..CHUNK_Y {
                for cz in 0..CHUNK_XZ {
                    let c_pos = Vec3::new(cx as f32, cy as f32, cz as f32) + world_position;
                    let block_type = self.get_block_type(c_pos);

                    chunk.set_block(cx, cy, cz, block_type);
                }
            }
        }
    }

    /// Gets the block type at this position
    pub fn get_block_type(&self, position: Vec3) -> VoxelType {
        /*  let height = 16.0
            + ((CHUNK_Y as f32)
                * self.noise_func.get_noise(
                    position.x / 16.0 * 1.5 + 0.001,
                    position.z / 16.0 * 1.5 + 0.001,
                ));

        if position.y <= 4.0 {
            return VoxelType::Stone(0.0);
        }

        if position.y <= height - 0.5 {
            VoxelType::Stone(0.0)
        } else if position.y > height + 0.5 {
            VoxelType::Air
        } else if position.y > height {
            VoxelType::Stone(position.y - height)
        } else {
            VoxelType::Stone(height - position.y)
        }*/

        let mut height = (CHUNK_Y as f32 / 2.0)
            * self.noise_func.get_noise(
                position.x / 64.0 * 1.5 + 0.001,
                position.z / 64.0 * 1.5 + 0.001,
            );

        height = (CHUNK_Y as f32 / 4.0) + height;
        
        if position.y <= 10.0 {
            return VoxelType::Stone(255.0);
        }
        
        if height > position.y {
            let mut diff = height - position.y;
            if diff <= 1.0 {
                diff = (Self::map_range((0.0, 1.0), (u8::MIN as f32, u8::MAX as f32), diff) as u8) as f32;
                
                return VoxelType::Stone(diff);
            }

            return VoxelType::Stone(255.0);
        } else {
           
            
            return VoxelType::Air;
        }

        /*  if height >= (position.y - 0.5) {

            // 36.5 >= 36.0

            let diff = height - position.y;
            if diff <= 1.0 {
                return VoxelType::Stone(0.5 - diff);
            }

            return VoxelType::Stone(1.0);
        } else {


            return VoxelType::Air;
        }*/

        /*  if position.y <= height - 0.5 {
            VoxelType::Stone(1.0)
        } else if position.y > height + 0.5 {
            VoxelType::Air
        } else if position.y > height {
            let diff = position.y - height;

            assert!(diff > 0., "diff = {diff}");
            assert!(diff <= 1., "diff = {diff}");

            VoxelType::Stone(diff)
        } else {
            let diff = height - position.y;

            assert!(diff > 0., "diff = {diff}");
            assert!(diff <= 1., "diff = {diff}");

            VoxelType::Stone(diff)
        }*/

        // println!("N: {raw_noise}");

        /*let terrain_noise = Self::map_range((-1.0, 1.0), (0.0, WORLD_HEIGHT as f32), raw_noise);

        if terrain_noise >= position.y {
            let mut diff = terrain_noise - position.y;
            if diff > 1.0 {
                diff = 0.0;
            }

            // diff = Self::map_range((0.0, 1.0), (u8::MIN as f32, u8::MAX as f32), diff);

            VoxelType::Stone(0.5)
        } else {
            VoxelType::Air
        }*/

        // Map this noise between 0 and world height
        //  let terrain_noise = Self::map_range((-1.0, 1.0), (0.0, WORLD_HEIGHT as f32), raw_noise);

        //   assert!(terrain_noise >= 0.0);
        //   assert!(terrain_noise <= WORLD_HEIGHT as f32);

        //println!("TN: {}", terrain_noise);

        //  let mut t = VoxelType::Air;

        // Calculate the density of the terrain at this point, 0.0 is air,
        // 1.0 is full underground, between these values is a range
        //   let density: f32;
        //   if terrain_noise > position.y - 1.0 {
        // 55.5 > 55-1
        // Fully underground
        //      density = 1.0;
        //   } else if terrain_noise < position.y + 1.0 {
        // 55.5 3042 < 55+1
        // Fully Aboveground
        //     density = 0.0;
        // } else {
        // Partial
        //      density = f32::abs(terrain_noise - position.y);
        //      println!("DN: {}", density);
        //  }

        // If the generated terrain noise is above our current height (or equal),
        // set the block to sand. This effectivity paints the world with voxels alongside
        // the noise
        // if terrain_noise >= position.y {
        //     assert!(density > 0.0);
        //     assert!(density <= 1.0);

        //     t = VoxelType::Stone(-1.0);
        // }

        //  if terrain_noise >= (position.y - 1.0) {
        //      t = VoxelType::Grass(-1.0);
        //  }

        // Get top layer grass
        //if t == VoxelType::Dirt(_) {
        //    if self.get_block_type(position + Vec3::new(0.0, 1.0, 0.0)) == VoxelType::Air {
        //        t = VoxelType::Grass;
        //    }
        // }

        //  t

        // Build noise
        /*let noise = self.noise_func.get_noise3d(
            position.x * 2. + 5.0,
            position.y * 2. + 3.0,
            position.z * 2. + 0.6,
        );

        //let noise = self
        //    .noise_func
        //    .get_noise(position.x * 2. + 5.0, position.z * 2. + 0.6);

        let v = position.y + noise;

        //  ahh
        let sn = 1f32
            - (position.x * position.x + position.y * position.y + position.z * position.z).sqrt()
                / 5f32;

        // println!("POS:  {}, V: {}, N: {}", position.y, v, sn);

        if v > 0.0 {
            VoxelType::Dirt(TerrainVoxel { density: noise })
        } else {
            VoxelType::Air
        }

        /*

          let up = Vec3::new(0.0, 1.0, 0.0);

         v *= 32.0;

         v += 12.0;

        let mut t = VoxelType::Air;

         if v >= position.y {
             t = VoxelType::Dirt(TerrainVoxel { density: v })
         }

         match t {
             // Get top layer grass
             VoxelType::Dirt(_) => {
                 if self.get_block_type(position + up) == VoxelType::Air {
                     t = VoxelType::Grass(TerrainVoxel { density: v });
                 }
             }
             // Replace air below water level with water
             VoxelType::Air => {
                 if position.y <= self.water_level {
                     t = VoxelType::Water;
                 }
             }
             _ => (),
         }

         // Bottom of the world should be dirt
         if position.y == 0.0 {
             t = VoxelType::Dirt(TerrainVoxel { density: 0.0 });
         }

         t*/*/
    }

    pub fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
        to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
    }
}
