trait Originator {
    fn generate_memento(&self) -> Box<Memento>;
    fn restore_from_memento(&mut self, &Memento);
}


trait Caretaker {
    fn add_memento(&mut self, Box<Memento>);
    fn get_memento(&mut self, usize) -> &Memento;
}


trait Memento {
    fn get_value(&self) -> usize;
}


#[derive(Debug)]
struct OriginatorX(usize);
impl Originator for OriginatorX {
    fn generate_memento(&self) -> Box<Memento>
    {
        Box::new(MementoX(self.0))
    }

    fn restore_from_memento(&mut self, m: &Memento)
    {
        self.0 = m.get_value()
    }
}


struct MementoX(usize);
impl Memento for MementoX {
    fn get_value(&self) -> usize
    {
        self.0
    }
}


struct CaretakerX {
    history: Vec<Box<Memento>>,
}


impl CaretakerX {
    fn new() -> CaretakerX {
        CaretakerX {
            history: Vec::new(),
        }
    }
}


impl Caretaker for CaretakerX {
    fn add_memento(&mut self, m: Box<Memento>)
    {
        self.history.push(m)
    }

    fn get_memento(&mut self, index: usize) -> &Memento
    {
        & *self.history[index]
    }
}

fn main()
{
    let mut caretaker  = CaretakerX::new();
    let mut originator = OriginatorX(10);

    caretaker.add_memento(originator.generate_memento());
    println!("{:?}", originator);
    originator.0 = 99;
    println!("{:?}", originator);
    originator.restore_from_memento(caretaker.get_memento(0));
    println!("{:?}", originator);
}