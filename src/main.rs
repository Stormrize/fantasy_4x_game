mod camera;
mod input;
mod terrain;
mod utils_debug;
mod network;
mod ui;

use::bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use crate::camera::camera::CameraPlugin;
use crate::terrain::renderer::MapPlugin;
use crate::ui::UiPlugin;

fn main() {
    println!("Start");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(CameraPlugin)
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //.add_plugins(LogDiagnosticsPlugin::default())
        .run();
    println!("End");
}

// fn main() {
//     // 1. Задаем исходные точки (сайты)
//     let sites = vec![
//         Point { x: 0.0, y: 0.0 },
//         Point { x: 1.0, y: 0.0 },
//         Point { x: 0.0, y: 1.0 }
//     ];
//
//     // 2. Строим диаграмму с помощью VoronoiBuilder
//     let my_voronoi = VoronoiBuilder::default()
//         .set_sites(sites)
//         // Задаем квадратную рамку размером 4.0, центрированную в (0,0)
//         .set_bounding_box(BoundingBox::new_centered_square(4.0))
//         // Опционально: делаем 5 итераций сглаживания Ллойда
//         .set_lloyd_relaxation_iterations(0)
//         .build()
//         .unwrap();
//
//     // 3. Как читать результат:
//
//     // Вариант А: Обход всех ячеек через итератор
//     my_voronoi.iter_cells().for_each(|cell| {
//         // Получаем координаты вершин (углов) конкретной ячейки
//         let vertices: Vec<&Point> = cell.iter_vertices().collect();
//         println!("Координаты вершин ячейки: {:?}", vertices);
//     });
//
//     // Вариант Б: Запрос конкретной ячейки по индексу (например, для второй точки)
//     let my_cell = my_voronoi.cell(1);
//     println!(
//         "Ячейка №1 имеет центр в точке {:?}, вершины Вороного: {:?}",
//         my_cell.site_position(),
//         my_cell.iter_vertices().collect::<Vec<&Point>>()
//     );
//
//     // Вариант В: Для графических движков (если нужны сырые индексные буферы)
//     let all_vertices = my_voronoi.vertices(); // Массив всех точек-вершин диаграммы
//     let indexed_cells = my_voronoi.cells();    // Массив массивов с индексами
//
//     // Получаем позицию первой вершины для первой ячейки
//     let first_vertex_idx = indexed_cells[0][0];
//     println!("Первая вершина первой ячейки: {:?}", all_vertices[first_vertex_idx]);
// }