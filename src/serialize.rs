use super::storage::ShapeStorage;

pub trait Serialize {
    fn serialize(&self) -> String;
}

impl Serialize for ShapeStorage {
    // TODO dynamically set with and height from screen size
    fn serialize(&self) -> String {
        let mut contents: Vec<String> = self.iter().map(|i| i.serialize()).collect();

        format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>
<!-- Created with Pizarra (https://github.com/categulario/pizarra) -->

<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
<svg
  xmlns:cc=\"http://creativecommons.org/ns#\"
  xmlns:svg=\"http://www.w3.org/2000/svg\"
  xmlns=\"http://www.w3.org/2000/svg\"
  width=\"400px\"
  height=\"400px\"
  version=\"1.1\"
  id=\"svg8\"
>
  <g id=\"layer1\">
    {}
  </g>
</svg>", contents.join("\n"))
    }
}
