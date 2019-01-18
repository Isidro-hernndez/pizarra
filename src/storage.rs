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

    pub fn serialize(&self) -> Vec<u8> {
        String::from("<xml></xml>").bytes().collect()
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
