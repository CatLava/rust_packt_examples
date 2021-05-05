use std::fmt::Display;

pub enum Property{
    Simple(&static str,String),
    Style(&static str,String),
    Transform(String),
}

//svg is similar to html
pub struct SvgTag{
    pub kind: &'static str,
    pub properties:Vec<Property>,
    pub children: Vec<SvgTag>,
}

impl SvgTag{
    pub fn new(kind:&'static str)->Self{
        SvgTag{
            kind,
            properties:Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn property<V:Display>(mut self, k:&'static str, v: V)-> Self {
        // takes ownership of self before changing it
        self.properties.push(Property::Simple(k,v.to_string()));
        self
    }

    pub fn style<V:Display>(mut self, k:&'static str, v: V)-> Self {
        // takes ownership of self before changing it
        self.properties.push(Property::Style(k,v.to_string()));
        self
    }

    pub fn x<V:Display>(self, v:V)-> Self{
        self.property(x,v)
    }

    pub fn y<V:Display>(self, v:V)-> Self{
        self.property(y,v)
    }
    pub fn width<V:Display>(self, v:V)-> Self{
        self.property(width,v)
    }
    pub fn height<V:Display>(self, v:V)-> Self{
        self.property(height,v)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let a = SvgTag::new("svg")
        assert_eq!(2 + 2, 4);
    }
}
