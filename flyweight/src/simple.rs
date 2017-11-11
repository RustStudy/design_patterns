use std::collections::HashMap;


#[derive(Debug)]
struct Object(String, usize);


struct Fryweight {
    pool: HashMap<String, Object>,
    counter: usize,
}


impl Fryweight {
    fn new() -> Fryweight
    {
        Fryweight {
            pool: HashMap::new(),
            counter: 0,
        }
    }

    fn obtain_object(&mut self, key: String) -> &mut Object
    {
        if self.pool.contains_key(&key) {
            return self.pool.get_mut(&key).unwrap()
        }

        self.pool.insert(key.clone(), Object(key.clone(), self.counter));
        self.counter += 1;
        self.obtain_object(key)
    }
}


fn main()
{
    let mut fryweight = Fryweight::new();

    {
        let o1 = fryweight.obtain_object("hoge".to_string());
        println!("{:?}", o1);
        o1.1 = 567;
    }

    let o2 = fryweight.obtain_object("hoge".to_string());
    println!("{:?}", o2);
}