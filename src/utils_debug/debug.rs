// use std::io::Write;
// use std::fs::OpenOptions;
// use noise::Perlin;
// use voronoice::Point;
// use crate::terrain::enums::biome::BiomeType;
// use crate::terrain::enums::resources::ResourceType;
// use crate::terrain::generation::cities::evaluate_rating::evaluate_rating;
// use crate::terrain::generation::resources::resource_generation::ResourceMap;
// use crate::terrain::generation::terrain::terrain_generation::TerrainMap;
// use crate::terrain::terrain_config::TERRAIN_SIZE;
//
// pub fn print_terrain_map(terrain: &TerrainMap) {
//
//     let mut file = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open("output.txt")
//         .expect("Не удалось открыть файл");
//
//
//     for biomes in terrain.map.iter() {
//         for biome in biomes.iter() {
//             match *biome {
//                 BiomeType::Water => write!(file, "~ ").expect("error"),
//                 BiomeType::Grass => write!(file, "0 ").expect("error"),
//                 BiomeType::Forest => write!(file, "f ").expect("error"),
//                 BiomeType::Hills => write!(file, "^ ").expect("error"),
//                 _ => {write!(file, "error").expect("error")},
//             }
//         }
//         write!(file, "\n").expect("error");
//     }
// }
//
// pub fn print_resource_map(resources: &ResourceMap) {
//
//     let mut file = OpenOptions::new()
//         .create(true) // создать, если нет
//         .append(true) // запись в конец
//         .open("output.txt")
//         .expect("Не удалось открыть файл");
//
//     write!(file, "\n\n\n\n\n\n\n\n\n\n\n").expect("error");
//
//     for i in resources.map.iter() {
//         for resource in i.iter() {
//             match resource {
//                 ResourceType::Food => write!(file, "1 ").expect("error"),
//                 ResourceType::Wood => write!(file, "2 ").expect("error"),
//                 ResourceType::Stone => write!(file, "3 ").expect("error"),
//                 ResourceType::MagicShards => write!(file, "4 ").expect("error"),
//                 ResourceType::None => write!(file, "0 ").expect("error"),
//             }
//         }
//         write!(file, "\n").expect("error");
//     }
// }
//
//
// // pub fn print_spawn_points(terrain: &TerrainMap, resources: &ResourceMap) {
// //     let points = evaluate_rating(terrain, resources);
// //     let race_coord = races_coord(terrain, resources);
// //
// //
// //
// //     let mut file = OpenOptions::new()
// //         .create(true)
// //         .append(true)
// //         .open("output.txt")
// //         .expect("Не удалось открыть файл");
// //
// //     write!(file, "\n\n\n\n\n\n\n\n\n\n\n").expect("error");
// //
// //     for col in 0..TERRAIN_SIZE {
// //         for row in 0..TERRAIN_SIZE {
// //             if race_coord.contains(&Point {y: col  as f64, x: row as f64}) {
// //                 write!(file, "* ").expect("error");
// //             } else if points.contains(&(col as u16, row as u16)) {
// //                 write!(file, "+ ").expect("error");
// //             } else {
// //                 write!(file, "- ").expect("error");
// //             }
// //         }
// //         write!(file, "\n").expect("error");
// //     }
// // }
//
//
// // pub fn print_tile_ratings(terrain: &TerrainMap, resources: &ResourceMap) {
// //
// //     let tile_ratings = rate_the_tiles(&terrain, &resources);
// //
// //     let mut file = OpenOptions::new()
// //         .create(true) // создать, если нет
// //         .append(true) // запись в конец
// //         .open("output.txt")
// //         .expect("Не удалось открыть файл");
// //
// //     write!(file, "\n\n\n\n\n\n\n\n\n\n\n").expect("error");
// //
// //     let mut min = 0;
// //     let mut max = 0;
// //
// //     for tiles in tile_ratings {
// //         for tile in tiles {
// //             if tile < min {
// //                 min = tile;
// //             }
// //             if tile > max {
// //                 max = tile;
// //             }
// //             write!(file, "{} ", tile).expect("error");
// //         }
// //         write!(file, "\n").expect("error");
// //     }
// //     println!("min: {}, max: {}", min, max);
// // }