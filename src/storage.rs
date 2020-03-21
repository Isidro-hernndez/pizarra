use super::poly::Shape;

pub struct ShapeStorage {
    contents: Vec<Box<dyn Shape>>,
}

impl ShapeStorage {
    pub fn new() -> ShapeStorage {
        ShapeStorage {
            contents: Vec::new(),
        }
    }

    pub fn iter(&self) -> ShapeIterator {
        ShapeIterator{
            iterator: self.contents.iter(),
        }
    }

    pub fn add(&mut self, shape: Box<dyn Shape>) {
        self.contents.push(shape);
    }

    pub fn last_mut(&mut self) -> Option<&mut Box<dyn Shape>> {
        self.contents.last_mut()
    }

    pub fn pop(&mut self) -> Option<Box<dyn Shape>> {
        self.contents.pop()
    }
}

pub struct ShapeIterator<'a> {
    iterator: std::slice::Iter<'a, Box<dyn Shape>>,
}

impl <'a> Iterator for ShapeIterator<'a> {
    type Item = &'a Box<dyn Shape>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_shapes_at_zoom() {
        assert!(false, "return an identifier of the shape");
    }

    #[test]
    fn test_last_mut() {
        assert!(false, "return a mutable reference to the last inserted shape");
    }

    #[test]
    fn test_remove_by_coordinate_radius_and_zoom_range() {
        assert!(false, "returns enough data to put the shapes again in their places");
    }

    #[test]
    fn test_remove_by_id() {
        assert!(false, "given a shape identifier remove it and return enough info to add it again");
    }

    #[test]
    fn test_iter_by_zoom_and_bounds() {
        assert!(false, "return an iterator over all the shapes that are visible given a zoom level and a bbox");
        assert!(false, "consider returning an iterator over draw commands instead of shapes");
        assert!(false, "each command must include a relative zoom level and a relative location");
    }

    #[test]
    fn test_relative_coordinates() {
        assert!(false, "at each zoom level coordinates should be relative, such that then rendering from it no transformation will be needed");
    }
}
