trait Subject {
    fn get_something(&mut self) -> usize;
}

struct RealSubject(usize);
impl RealSubject {
    fn new() -> RealSubject
    {
        let mut rs = RealSubject(0);
        rs.load_something();

        rs
    }

    fn load_something(&mut self)
    {
        println!("Try to load something, it is extremely heavy.");

        self.0 = 100;
    }
}

impl Subject for RealSubject {
    fn get_something(&mut self) -> usize
    {
        self.0
    }
}

struct Proxy(Option<RealSubject>);
impl Proxy {
    fn new() -> Proxy
    {
        Proxy(None)
    }
}
impl Subject for Proxy {
    fn get_something(&mut self) -> usize
    {
        match self.0 {
            Some(ref mut something) => {
                something.get_something()
            },
            None => {
                let mut rs = RealSubject::new();
                let x = rs.get_something();
                self.0 = Some(rs);
                x
            }
        }
    }
}


fn main()
{
    let mut rs = RealSubject::new();
    println!("Create RealSubject");
    println!("{}", rs.get_something());

    let mut p1 = Proxy::new();
    println!("Create Proxy Object");
    let mut p2 = Proxy::new();
    println!("Create Proxy Object");
    println!("{}", p1.get_something());
    println!("{}", p2.get_something());
}