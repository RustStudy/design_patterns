/*
抽象工厂模式
*/

trait AbstractFactory<'a> {
    fn create_product_x(&self) -> Box<ProductX + 'a>;
    fn create_product_y(&self) -> Box<ProductY + 'a>;
}


trait ProductX {
    fn get_value(&self) -> String;
}


trait ProductY {
    fn get_value(&self) -> String;
}


struct ConcreteProductX(String);
impl ConcreteProductX {
    fn new(msg: String) -> ConcreteProductX {
        ConcreteProductX(msg + &" ProductX".to_string())
    }
}


impl ProductX for ConcreteProductX {
    fn get_value(&self) -> String
    {
        self.0.clone()
    }
}


struct ConcreteProductY(String);
impl ConcreteProductY {
    fn new(msg: String) -> ConcreteProductY {
        ConcreteProductY(msg + &" ProductY".to_string())
    }
}


impl ProductY for ConcreteProductY {
    fn get_value(&self) -> String
    {
        self.0.clone()
    }
}


struct ConcreteFactoryA;
impl<'a> AbstractFactory<'a> for ConcreteFactoryA {
    fn create_product_x(&self) -> Box<ProductX + 'a>
    {
        Box::new(ConcreteProductX::new("FactoryA".to_string())) as Box<ProductX>
    }

    fn create_product_y(&self) -> Box<ProductY + 'a>
    {
        Box::new(ConcreteProductY::new("FactoryA".to_string())) as Box<ProductY>
    }
}


struct ConcreteFactoryB;
impl<'a> AbstractFactory<'a> for ConcreteFactoryB {
    fn create_product_x(&self) -> Box<ProductX + 'a>
    {
        Box::new(ConcreteProductX::new("FactoryB".to_string())) as Box<ProductX>
    }

    fn create_product_y(&self) -> Box<ProductY + 'a>
    {
        Box::new(ConcreteProductY::new("FactoryB".to_string())) as Box<ProductY>
    }
}



enum FactoryID {
    A,
    B,
}

fn create_factory<'a>(id: FactoryID) -> Box<AbstractFactory<'a> + 'a>
{
    match id {
        FactoryID::A => Box::new(ConcreteFactoryA),
        FactoryID::B => Box::new(ConcreteFactoryB),
    }
}



fn main()
{
    let factory_a = create_factory(FactoryID::A);
    let a_x = factory_a.create_product_x();
    let a_y = factory_a.create_product_y();
    println!("{}", a_x.get_value());
    println!("{}", a_y.get_value());

    let factory_b = create_factory(FactoryID::B);
    let b_x = factory_b.create_product_x();
    let b_y = factory_b.create_product_y();
    println!("{}", b_x.get_value());
    println!("{}", b_y.get_value());
}

