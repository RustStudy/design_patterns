fn worker_a()
{
    println!("hoge");
}


fn worker_b()
{
    println!("huga");
}


fn worker_c()
{
    println!("piyo");
}


struct Facade;
impl Facade {
    fn facade_method(&self)
    {
        worker_a();
        worker_b();
        worker_c();
    }
}


fn main()
{
    let f = Facade;
    f.facade_method();
}