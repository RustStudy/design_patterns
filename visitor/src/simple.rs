trait Visitor {
    fn visit(&self, &Acceptor);
}


trait Acceptor {
    fn accept(&self, &Visitor);
    fn get_value(&self) -> &String;
}


struct VisitorX;
impl Visitor for VisitorX {
    fn visit(&self, a: &Acceptor)
    {
        println!("VisitorX - Acceptor {}", a.get_value());
    }
}

struct AcceptorX(String);
impl Acceptor for AcceptorX {
    fn accept(&self, v: &Visitor)
    {
        v.visit(self);
    }


    fn get_value(&self) -> &String
    {
        &self.0
    }
}

fn main()
{
    let v = VisitorX;
    let a1 = AcceptorX("Number 1".to_string());
    let a2 = AcceptorX("Number 2".to_string());

    a1.accept(&v);
    a2.accept(&v);
}