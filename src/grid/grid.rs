use super::bbox::{BBox, BlockBBox, BLOCKS_IN_REGION};


pub type Uheight = u16;


pub struct Grid {
    bbox: BlockBBox,
    heights: Vec<Uheight>,
}

impl Grid {
    pub fn new(bbox: BBox) -> Self {
        let heights = vec![0; (bbox.width() * bbox.height()) as usize * BLOCKS_IN_REGION as usize * BLOCKS_IN_REGION as usize];
        Self {
            bbox: bbox.into(),
            heights
        }
    }

    pub fn get_bbox(&self) -> &BlockBBox {
        &self.bbox
    }

    pub fn get_x_idx(&self, x: i32) -> usize {
        (x - self.bbox.x_0) as usize
    }

    pub fn get_z_idx(&self, z: i32) -> usize {
        (z - self.bbox.z_0) as usize
    }


    pub fn get_height(&self, x: i32, z: i32) -> Uheight {
        self.heights[self.get_z_idx(z) * self.bbox.width() as usize + self.get_x_idx(x)]
    }

    pub fn set_height(&mut self, x: i32, z: i32, height: Uheight) {
        let idx = self.get_z_idx(z) * self.bbox.width() as usize + self.get_x_idx(x);
        self.heights[idx] = height;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn grid_bbox() {
        let grid = Grid::new(BBox::new(-1, -1, 1, 1));

        assert_eq!(grid.get_bbox().width(), 2*1024);
        assert_eq!(grid.get_bbox().height(), 2*1024);
    }

    #[test]
    fn coord_to_idx() {
        let grid = Grid::new(BBox::new(-1, -1, 1, 1));

        assert_eq!(grid.get_x_idx(-1024), 0);
        assert_eq!(grid.get_z_idx(-1024), 0);

        assert_eq!(grid.get_x_idx(0), 1024);
        assert_eq!(grid.get_z_idx(0), 1024);

        assert_eq!(grid.get_x_idx(1023), 1023 + 1024);
        assert_eq!(grid.get_z_idx(1023), 1023 + 1024);
    }

    #[test]
    fn get_set_height() {
        let mut grid = Grid::new(BBox::new(-1, -1, 1,1));

        grid.set_height(0, 0, 20);

        assert_eq!(grid.get_height(0,0), 20);

    }
}
