use bevy::prelude::Resource;
use voronoice::{BoundingBox, Point, VoronoiBuilder};
use crate::terrain::enums::buildings::BuildingType;
use crate::terrain::enums::resources::ResourceType;
use crate::terrain::generation::cities::evaluate_rating::evaluate_rating;
use crate::terrain::generation::resources::resource_generation::ResourceMap;
use crate::terrain::generation::terrain::terrain_generation::TerrainMap;
use crate::terrain::terrain_config::TerrainConfig;


// TODO не имплиментированно
#[derive(Resource)]
pub struct BuildingMap {
    //pub map: [[BuildingType; TERRAIN_SIZE]; TERRAIN_SIZE],
    pub map: Vec<BuildingType>,
    pub size: usize,
}



impl BuildingMap {
    pub fn new(config: &TerrainConfig, terrain: &TerrainMap, resources: &ResourceMap) -> Self {
        let race_coord = race_coord(terrain, resources, config);
        println!("race coord len: {}", race_coord.len());
        let terrain_size = config.terrain_size;

        let mut building_points: Vec<BuildingType> = vec![BuildingType::None; terrain_size * terrain_size];
        
        for i in race_coord {
            let x = i.x as usize;
            let y = i.y as usize;
            let index = y * terrain_size + x;
            building_points[index] = BuildingType::Capital;
        }
        Self {
            map: building_points,
            size: terrain_size,
        }
    }

    #[inline]
    pub fn get(
        &self,
        col: usize,
        row: usize,
    ) -> BuildingType {
        debug_assert!(col < self.size);
        debug_assert!(row < self.size);

        let index = row * self.size + col;
        self.map[index]
    }
}

pub fn race_coord(terrain: &TerrainMap, resources: &ResourceMap, config: &TerrainConfig) -> Vec<Point> {
    let building_points = evaluate_rating(terrain, resources, config);

    let distance = |p1: &(u16, u16), p2: &(u16, u16)| -> f32 {
        let dx = p1.0 as f32 - p2.0 as f32;
        let dy = p1.1 as f32 - p2.1 as f32;
        (dx * dx + dy * dy).sqrt()
    };

    let mut selected_indices: Vec<usize> = Vec::new();

    let mut races_coord: Vec<Point> = Vec::new();

    let mut max_dist = -1.0;
    let mut best_pair = (0, 0);

    for i in 0..building_points.len() {
        for j in (i + 1)..building_points.len() {
            let d = distance(&building_points[i], &building_points[j]);
            if d > max_dist {
                max_dist = d;
                best_pair = (i, j);

            }
        }
    }

    races_coord.push(Point {
            y: building_points[best_pair.0].0 as f64,
            x: building_points[best_pair.0].1 as f64,
    });

    races_coord.push(Point {
            y: building_points[best_pair.1].0 as f64,
            x: building_points[best_pair.1].1 as f64,
    });

    selected_indices.push(best_pair.0);
    selected_indices.push(best_pair.1);


    while selected_indices.len() < 4 {
        let mut best_next_point = 0;
        let mut max_of_min_distances = -1.0;

        for k in 0..building_points.len() {
            if selected_indices.contains(&k) {
                continue;
            }

            let min_dist_to_selected = selected_indices
                .iter()
                .map(|&sel| distance(&building_points[k], &building_points[sel]))
                .fold(f32::INFINITY, |acc, x| acc.min(x));

            if min_dist_to_selected > max_of_min_distances {
                max_of_min_distances = min_dist_to_selected;
                best_next_point = k;
            }
        }

        races_coord.push(Point {
                y: building_points[best_next_point].0 as f64,
                x: building_points[best_next_point].1 as f64,
            });
        selected_indices.push(best_next_point);

    }
    
    races_coord
}




//
// pub fn voronoi_diagram(terrain: &TerrainMap, resources: &ResourceMap) {
//
//     let sites = race_coord(terrain, resources);
//
//     let center = Point { x: (TERRAIN_SIZE / 2) as f64, y: (TERRAIN_SIZE / 2) as f64 };
//
//     let bbox = BoundingBox::new(center, TERRAIN_SIZE as f64, TERRAIN_SIZE as f64);
//
//
//     let my_voronoi = VoronoiBuilder::default()
//         .set_sites(sites)
//         .set_bounding_box(bbox)
//         .build()
//         .unwrap();
//
//     my_voronoi.iter_cells().for_each(|cell| {
//         println!("Vertices of cell: {:?}", cell.iter_vertices().collect::<Vec<&Point>>());
//         println!("site position: {:?}", cell.site_position());
//     });
// }