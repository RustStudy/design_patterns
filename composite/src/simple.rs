trait Composite {
    fn get_name(&self) -> String;
    fn get_child(&self) -> Option<&Box<Composite>>;
    fn set_child(&mut self, Box<Composite>);
    fn print_child_name_recursive(&self);
}


struct File {
    name: String,
    child: Option<Box<Composite>>,
}


impl File {
    fn new(name: String) -> File
    {
        File {
            name: name,
            child: None,
        }
    }
}


impl Composite for File {
    fn get_name(&self) -> String
    {
        self.name.clone()
    }

    fn get_child(&self) -> Option<&Box<Composite>>
    {
        match self.child {
            Some(ref x) => Some(x),
            None        => None,
        }
    }

    fn set_child(&mut self, c: Box<Composite>)
    {
        self.child = Some(c)
    }

    fn print_child_name_recursive(&self)
    {
        print!(" -> {}", self.get_name());
        if let Some(x) = self.get_child() {
            x.print_child_name_recursive();
        } else {
            println!("");
        }
    }
}


// A directory is a file.
struct Directory {
    f: File,
}


impl Directory {
    fn new(name: String) -> Directory
    {
        Directory {
            f: File::new(name),
        }
    }
}


impl Composite for Directory {
    fn get_name(&self) -> String
    {
        self.f.get_name()
    }

    fn get_child(&self) -> Option<&Box<Composite>>
    {
        self.f.get_child()
    }

    fn set_child(&mut self, c: Box<Composite>)
    {
        self.f.set_child(c)
    }


    fn print_child_name_recursive(&self)
    {
        self.f.print_child_name_recursive();
    }
}



fn main()
{
    let mut d1 = Directory::new("root".to_string());
    let mut d2 = Directory::new("boot".to_string());
    let f1 = File::new("vmlinuz-linux".to_string());

    d2.set_child(Box::new(f1));
    d1.set_child(Box::new(d2));

    println!("Start");
    d1.print_child_name_recursive();
}