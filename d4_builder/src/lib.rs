// Using builder patterns to construct code

use std::fmt::Display;

#[derive(Debug,PartialEq)]
pub enum Property {
    Simple(&' static str, String),
    Style(&' static str,String),
    Transform(String),
}

//svg is similar to html
// Building this out for test cases
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

    pub fn child(mut self, c: SvgTag) -> Self {
        self.children.push(c);
        self
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
        self.property("x",v)
    }

    pub fn y<V:Display>(self, v:V)-> Self{
        self.property("y",v)
    }
    pub fn w<V:Display>(self, v:V)-> Self{
        self.property("width",v)
    }
    pub fn h<V:Display>(self, v:V)-> Self{
        self.property("height",v)
    }
}

#[cfg(test)]
mod tests {
    use Super::*;
    #[test]
    fn it_works() {
        let a = SvgTag::new("svg")
            .w("60px")
            .h("80px")
            .child(SvgTag::new("rect").x(5).y(5).w(50).h(20));
        let b = SvgTag{}

        assert_eq!(a);

    }
}
