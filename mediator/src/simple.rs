use std::collections::HashMap;


struct Mediator {
    colleagues: HashMap<String, Colleague>,
}


impl Mediator {
    fn new() -> Mediator {
        Mediator {
            colleagues: HashMap::new(),
        }
    }

    fn add_colleague(&mut self, c: Colleague)
    {
        self.colleagues.insert(c.0.clone(), c);
    }

    fn consult_to(&self, s: &String, msg: String)
    {
        self.colleagues.get(s).unwrap().receive_msg(msg);
    }

    fn get(&self, s: &String) -> &Colleague
    {
        self.colleagues.get(s).unwrap()
    }
}


struct Colleague(String);
impl Colleague {
    fn new(s: &String) -> Colleague
    {
        Colleague(s.clone())
    }

    fn send_msg(&self, m: &Mediator, to: &String, msg: String)
    {
        m.consult_to(to, msg);
    }

    fn receive_msg(&self, msg: String)
    {
        println!("{} gets {}", self.0, msg);
    }
}


fn main()
{
    let mut mediator = Mediator::new();
    let key1 = "Hoge".to_string();
    let c1 = Colleague::new(&key1);
    let key2 = "Piyo".to_string();
    let c2 = Colleague::new(&key2);

    mediator.add_colleague(c1);
    mediator.add_colleague(c2);

    let c1 = mediator.get(&key1);
    c1.send_msg(&mediator, &key2, "hi from Hoge".to_string());
    let c2 = mediator.get(&key2);
    c2.send_msg(&mediator, &key1, "hi from Piyo".to_string());
}