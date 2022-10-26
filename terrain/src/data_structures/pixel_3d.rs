use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use rust_3d::*;

use crate::model::pixel::Pixel;
#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
pub struct Pixel3D {
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub moisture: f64,
    pub habitance: bool,
}

impl Pixel3D {
    pub fn new(x: f64, y: f64, pixel: &Pixel) -> Self {
        Pixel3D {
            x,
            y,
            height: pixel.height,
            moisture: pixel.moisture,
            habitance: pixel.habitance,
        }
    }
}

impl Eq for Pixel3D {}

impl Ord for Pixel3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Pixel3D::default();
        sqr_dist_3d(&origin, self)
            .partial_cmp(&sqr_dist_3d(&origin, other))
            .unwrap_or(Ordering::Equal)
    }
}

impl IsND for Pixel3D {
    fn n_dimensions() -> usize {
        3
    }

    fn position_nd(&self, dimension: usize) -> Result<f64> {
        match dimension {
            0 => Ok(self.x),
            1 => Ok(self.y),
            2 => Ok(self.height),
            _ => Err(ErrorKind::IncorrectDimension),
        }
    }
}
impl Is3D for Pixel3D {
    #[inline(always)]
    fn x(&self) -> f64 {
        self.x
    }
    #[inline(always)]
    fn y(&self) -> f64 {
        self.y
    }
    #[inline(always)]
    fn z(&self) -> f64 {
        self.height
    }
}

impl IsBuildableND for Pixel3D {
    fn new_nd(coords: &[f64]) -> Result<Self> {
        if coords.len() != 3 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Pixel3D {
            x: coords[0],
            y: coords[1],
            height: coords[2],
            moisture: 0f64,
            habitance: true,
        })
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()>
    where
        P: IsBuildableND,
    {
        if P::n_dimensions() != 3 {
            return Err(ErrorKind::DimensionsDontMatch);
        }

        self.x = other.position_nd(0)?;
        self.y = other.position_nd(1)?;
        self.height = other.position_nd(2)?;
        Ok(())
    }
}

impl Hash for Pixel3D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.height.to_bits().hash(state);
    }
}

impl IsBuildable3D for Pixel3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Pixel3D {
            x,
            y,
            height: z,
            moisture: 0f64,
            habitance: true,
        }
    }

    fn from<P>(&mut self, other: &P)
    where
        P: Is3D,
    {
        self.x = other.x();
        self.y = other.y();
        self.height = other.z();
    }
}

impl IsEditableND for Pixel3D {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        match dimension {
            0 => self.x = val,
            1 => self.y = val,
            2 => self.height = val,
            _ => return Err(ErrorKind::DimensionsDontMatch),
        }
        Ok(())
    }
}

impl IsEditable3D for Pixel3D {
    #[inline(always)]
    fn set_x(&mut self, val: f64) {
        self.x = val;
    }

    #[inline(always)]
    fn set_y(&mut self, val: f64) {
        self.y = val;
    }

    #[inline(always)]
    fn set_z(&mut self, val: f64) {
        self.height = val;
    }
}
