trait Implementor {
    fn decorate(&self, String) -> String;
}


struct ParenImpl;
impl Implementor for ParenImpl {
    fn decorate(&self, msg: String) -> String
    {
        "(".to_string() + &msg + &")".to_string()
    }
}


struct BracketImpl;
impl Implementor for BracketImpl {
    fn decorate(&self, msg: String) -> String
    {
        "{".to_string() + &msg + &"}".to_string()
    }
}


struct Abstraction<'a> {
    implementer: &'a Implementor
}
impl<'a> Abstraction<'a> {
    fn new(i: &Implementor) -> Abstraction {
        Abstraction {
            implementer: i,
        }
    }

    fn convert(&self, msg: String) -> String
    {
        self.implementer.decorate(msg)
    }
}

struct RefinedAbstraction<'a> {
    abstraction: Abstraction<'a>,
}
impl<'a> RefinedAbstraction<'a> {
    fn new(i: &Implementor) -> RefinedAbstraction {
        RefinedAbstraction {
            abstraction: Abstraction::new(i)
        }
    }

    fn convert(&self, msg: String) -> String
    {
        self.abstraction.convert(msg)
    }

    fn print_convert_msg(&self, msg: String)
    {
        println!("{}", self.abstraction.convert(msg));
    }
}


fn main()
{
    let paren_impl   = &ParenImpl;
    let bracket_impl = &BracketImpl;

    let abst_p = RefinedAbstraction::new(paren_impl as &Implementor);
    let abst_b = RefinedAbstraction::new(bracket_impl as &Implementor);

    println!("{}", abst_p.convert("YOYO".to_string()));
    abst_b.print_convert_msg("oops".to_string());
}