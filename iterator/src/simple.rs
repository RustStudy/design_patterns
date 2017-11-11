struct Container<T: Sized + Copy> {
    buf: Vec<T>,
    index: usize,
}


impl<T: Copy> Container<T> {
    fn new() -> Container<T>
    {
        Container {
            buf: Vec::new(),
            index: 0,
        }
    }

    fn add(&mut self, t: T)
    {
        self.buf.push(t);
    }
}


impl<T: Copy> Iterator for Container<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
        match self.index < self.buf.len() {
            true  => {
                let t = Some(self.buf[self.index]);
                self.index += 1;
                t
            }
            false => None,
        }
    }
}


fn main()
{
    let mut c: Container<usize> = Container::<usize>::new();
    c.add(10);
    c.add(20);
    c.add(30);

    for i in c {
        println!("{}", i);
    }
}