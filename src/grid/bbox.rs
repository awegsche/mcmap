use std::fmt::Display;

pub const BLOCKS_IN_CHUNK: u32 = 32;
pub const BLOCKS_IN_REGION: u32 = BLOCKS_IN_CHUNK * 32;

pub struct BBox {
    pub x_0: i32,
    pub z_0: i32,
    pub x_1: i32,
    pub z_1: i32,
}

// BoundingBox in Block coords
pub struct BlockBBox {
    pub x_0: i32,
    pub z_0: i32,
    pub x_1: i32,
    pub z_1: i32,
}

// -------------------------------------------------------------------------------------------------
// ---- impls --------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

// ---- BBox ---------------------------------------------------------------------------------------
//
impl BBox {
    pub fn new(x_0: i32, z_0: i32, x_1: i32, z_1: i32) -> Self {
        Self { x_0, z_0, x_1, z_1 }
    }

    pub fn width(&self) -> u32 {
        (self.x_1 - self.x_0) as u32
    }

    pub fn height(&self) -> u32 {
        (self.z_1 - self.z_0) as u32
    }
}

impl Display for BBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}) -- ({}, {})",
            self.x_0, self.z_0, self.x_1, self.z_1
        )
    }
}

// ---- BBox ---------------------------------------------------------------------------------------
//
impl BlockBBox {
    pub fn width(&self) -> u32 {
        (self.x_1 - self.x_0) as u32
    }

    pub fn height(&self) -> u32 {
        (self.z_1 - self.z_0) as u32
    }
}


impl From<BBox> for BlockBBox {
    fn from(value: BBox) -> Self {
        Self {
            x_0: value.x_0 * BLOCKS_IN_REGION as i32,
            z_0: value.z_0 * BLOCKS_IN_REGION as i32,
            x_1: value.x_1 * BLOCKS_IN_REGION as i32,
            z_1: value.z_1 * BLOCKS_IN_REGION as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blockbbox_from_bbox() {
        let bbox: BlockBBox = BBox::new(0,0,1,1).into();

        assert_eq!(bbox.x_0, 0);
        assert_eq!(bbox.x_1, 1024);
    }
}
