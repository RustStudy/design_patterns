type BinaryFn<T> = Fn(T, T) -> T;


struct Context<'a, T: 'a> {
    strategy: &'a BinaryFn<T>,
}


impl<'a, T> Context<'a, T> {
    fn new(f: &'a BinaryFn<T>) -> Context<'a, T>
    {
        Context {
            strategy: f,
        }
    }

    fn execute(&self, x: T, y: T) -> T
    {
        (*self.strategy)(x, y)
    }

    fn set_strategy(&mut self, f: &'a BinaryFn<T>)
    {
        self.strategy = f
    }
}


fn main() {
    let add = |x: usize, y: usize| x + y;
    let mul = |x: usize, y: usize| x * y;
    let div = |x: usize, y: usize| x / y;
    let and = |x: usize, y: usize| x & y;
    let mut c = Context::new(&add);

    println!("{:?}", c.execute(1, 2));

    c.set_strategy(&mul);
    println!("{:?}", c.execute(1, 2));

    c.set_strategy(&div);
    println!("{:?}", c.execute(2, 2));

    c.set_strategy(&and);
    println!("{:?}", c.execute(2, 2));
}