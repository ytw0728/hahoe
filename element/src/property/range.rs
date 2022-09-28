
///current position
pub trait Range {
    pub fn get_current_bounding_Box(&self) -> Result<BoundingBox3d>{
        Ok(
            BoundingBox3d::new(get_min, get_max)
        )
    }
    pub fn get_center_position(&self) -> Result<Point3d>;

    fn get_min(&self) -> Point3d;
    fn get_max(&self) -> Point3d;
}
