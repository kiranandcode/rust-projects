extern crate azul;

use azul::prelude::*;

struct DataModel {}

impl Layout for DataModel {
    fn layout(&self, _: WindowInfo<Self>) -> Dom<Self> {
        Dom::new(NodeType::Div)
    }
}


fn main() {
    let app = App::new(DataModel {}, AppConfig::default());
    let window = Window::new(WindowCreateOptions::default(), Css::native()).unwrap();

    app.run(window).unwrap();
    println!("Hello, world!");
}
