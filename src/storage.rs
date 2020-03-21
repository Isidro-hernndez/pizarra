use super::shape::Shape;

pub struct ShapeStorage {
    contents: Vec<Box<dyn Shape>>,
}

/// A storage struct that organizes shapes by their zoom level and allos for
/// fast queries given a zoom and a bbox.
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

    pub fn add(&mut self, shape: Box<dyn Shape>, zoom: i32) -> usize {
        unimplemented!()
    }

    pub fn remove(&mut self, id: usize) -> i32 {
        unimplemented!()
    }

    pub fn last_mut(&mut self) -> Option<&mut Box<dyn Shape>> {
        self.contents.last_mut()
    }

    pub fn pop(&mut self) -> Option<Box<dyn Shape>> {
        self.contents.pop()
    }

    pub fn shape_count(&self) -> usize {
        unimplemented!()
    }

    pub fn layer_count(&self) -> usize {
        unimplemented!()
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
    use super::ShapeStorage;
    use crate::shape::{Shape, Line, Rectangle, Circle};
    use crate::color::Color;

    #[test]
    fn test_add_shapes_at_zoom() {
        let mut storage = ShapeStorage::new();
        let shapes: Vec<(Box<dyn Shape>, i32)> = vec![
            (Box::new(Line::new(Color::red())), 0),
            (Box::new(Rectangle::new(Color::green())), 1),
            (Box::new(Circle::new(Color::blue())), -1),
        ];
        let ids: Vec<_> = shapes.into_iter().map(|(shape, zoom)| {
            storage.add(shape, zoom)
        }).collect();

        assert_eq!(storage.shape_count(), 3);
        assert_eq!(storage.layer_count(), 3);

        assert_eq!(ids[0], 1);
        assert_eq!(ids[1], 2);
        assert_eq!(ids[2], 3);
    }

    #[test]
    fn test_last_mut() {
        let mut storage = ShapeStorage::new();

        assert!(storage.last_mut().is_none());

        storage.add(Box::new(Line::new(Color::blue())), 0);

        let last_shape = storage.last_mut().unwrap();

        last_shape.handle([0.0, 0.0]);
        last_shape.handle([1.0, 0.0]);
    }

    #[test]
    #[ignore]
    fn test_remove_by_coordinate_radius_and_zoom() {
        assert!(false, "returns enough data to put the shapes again in their places");
        assert!(false, "the data is for restoring the shape in case of a ctrl-z");
        assert!(false, "only visible shapes given the zoom should be deleted.");
        assert!(false, "if there are shapes behind other shapes only delete the one in the front");
    }

    #[test]
    fn test_remove_by_id() {
        let mut storage = ShapeStorage::new();

        assert_eq!(storage.shape_count(), 0);

        let id = storage.add(Box::new(Line::new(Color::green())), 0);

        assert_eq!(storage.shape_count(), 1);

        let data = storage.remove(id);

        assert_eq!(storage.shape_count(), 0);

        assert!(false, "given a shape identifier remove it and return enough info to add it again");
        assert!(false, "this is to implement ctrl-z after adding a shape");
    }

    #[test]
    fn test_iter_by_zoom_and_bounds() {
        assert!(false, "return an iterator over all the shapes that are visible given a zoom level and a bbox");
        assert!(false, "consider returning an iterator over draw commands instead of shapes");
        assert!(false, "each command must include a relative zoom level and a relative location");
        assert!(false, "shapes at different layers are returned by their relative_layer");
    }

    #[test]
    fn test_relative_coordinates() {
        assert!(false, "at each zoom level coordinates should be relative, such that then rendering from it no transformation will be needed");
    }
}
