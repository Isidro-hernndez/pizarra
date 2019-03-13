use super::storage::ShapeStorage;

trait Serialize {
    fn serialize(&self) -> &str;
}

impl Serialize for ShapeStorage {
    fn serialize(&self) -> String {
        let header = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>
<!-- Created with Pizarra (https://github.com/categulario/pizarra) -->

<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
<svg
  xmlns:cc=\"http://creativecommons.org/ns#\"
  xmlns:svg=\"http://www.w3.org/2000/svg\"
  xmlns=\"http://www.w3.org/2000/svg\"
  width=\"40px\"
  height=\"40px\"
  version=\"1.1\"
  id=\"svg8\"
>
  <g id=\"layer1\">")


    }

    pub fn footer(&self) -> &'static[u8] {
        "  </g>
</svg>".as_bytes()
    }
    }
}
