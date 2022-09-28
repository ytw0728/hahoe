pub struct animal{
    position : Point3d,
    width : i32,    //x 
    length : i32,   //y
    height : i32,   //z
}

impl Range for animal {
    fn get_center_position(&self) -> Result<Point3d>{
        Ok(self.position)
    }

    fn get_min(&self) -> Point3d {
        Point3d::new(
            position.x - width / 2,
            position.y - length / 2, 
            position.z - height / 2,
        )
    }

    fn get_max(&self) -> Point3d {
        Point3d::new(
            position.x + width / 2,
            position.y + length / 2, 
            position.z + height / 2,
        )
    }
}