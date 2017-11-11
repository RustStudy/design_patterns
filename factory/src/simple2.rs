/*
工厂模式
*/ 

trait Product {
    fn convert(&self, String) -> String;
}


trait Factory {
    fn create_product(&self) -> Box<Product>;
    fn convert(&self, s: String) -> String
    {
        self.create_product().convert(s)
    }
}


struct ConcreteProductX;
impl Product for ConcreteProductX {
    fn convert(&self, s: String) -> String
    {
        s.to_uppercase()
    }
}


struct ConcreteFactoryX;
impl Factory for ConcreteFactoryX {
    fn create_product(&self) -> Box<Product>
    {
        Box::new(ConcreteProductX) as Box<Product>
    }
}


fn main()
{
    let f = ConcreteFactoryX;
    println!("{}", f.convert("hogehoge piyopiyo".to_string()))
}