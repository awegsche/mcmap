use std::{fmt::Display, ops::Index};

use ranvil;

mod grid;


use grid::grid::Grid;
use grid::bbox::*;


fn main() {
    println!("Hello, world!");

    if let Some(grid) = scan_save() {
        println!("Scanned save");
    } else {
        println!("Couldn't find");
    }
}




fn scan_save() -> Option<Grid> {
    let mut binding = ranvil::get_saves()?;
    let first_save = binding.first_mut()?;

    let coords = first_save
        .regions
        .iter()
        .map(|r| (r.get_x_coord(), r.get_z_coord()))
        .collect::<Vec<_>>();

    let x_min = coords.iter().map(|(x, _)| *x).min().unwrap();
    let x_max = coords.iter().map(|(x, _)| *x).max().unwrap();
    let z_min = coords.iter().map(|(_, z)| *z).min().unwrap();
    let z_max = coords.iter().map(|(_, z)| *z).max().unwrap();

    let bbox = BBox::new(x_min, z_min, x_max, z_max);

    for (x, z) in coords {
        println!("loading {} {}", x, z);
        first_save.load_region(x, z);
    }

    println!("{}, w: {}, h: {}", bbox, bbox.width(), bbox.height());

    let mut grid = Grid::new(bbox);

    Some(grid)
}
