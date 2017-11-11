/// the observable type
pub trait Observable<T> {
    fn register(&mut self, observer: Box<Observer<Item = T>>);
}
 
pub trait Observer {
    type Item;
 
    fn notify(&self, val: &Self::Item);
}
 
 
/// the actual structs which implement the Observable and Observer
/// traits
 
/// the specific Observable
pub struct EvenCounter {
    counter: u32,
    observers: Vec<Box<Observer<Item = u32>>>,
}
 
impl EvenCounter {
    pub fn new() -> Self {
        EvenCounter {
            counter: 0u32,
            observers: Vec::new(),
        }
    }
 
    pub fn run(&mut self) {
        loop {
            use std::thread;
            use std::time::Duration;
 
            thread::sleep(Duration::from_millis(self.get_rand_duration()));
 
            self.counter += 1;
 
            if self.counter % 2 == 0 {
                for observer in self.observers.iter() {
                    observer.notify(&self.counter);
                }
            }
        }
    }
 
    fn get_rand_duration(&self) -> u64 {
        if cfg!(target_os = "windows") {
            500u64
        } else {
            use std::process::Command;
            use std::str::FromStr;
 
            let rand_cmd = Command::new("sh")
                .arg("-c")
                .arg("echo $(( $RANDOM%1000 + 1 ))")
                .output()
                .expect("failed to get OS RNG");
 
            u64::from_str(&String::from_utf8(rand_cmd.stdout).unwrap().trim()).unwrap()
        }
    }
}
 
impl Observable<u32> for EvenCounter {
    fn register(&mut self, observer: Box<Observer<Item = u32>>) {
        self.observers.push(observer);
    }
}
 
/// the specific Observer type
pub struct EvenObserver {
    name: String,
}
 
impl EvenObserver {
    pub fn new(name: String) -> Self {
        EvenObserver { name: name }
    }
 
    fn name(&self) -> &str {
        &self.name
    }
}
 
impl Observer for EvenObserver {
    type Item = u32;
 
    fn notify(&self, val: &Self::Item) {
        println!("{} got {}", self.name(), val);
    }
}

fn main() {
    let mut foo = EvenCounter::new();
    let (bar, baz, quux) = (Box::new(EvenObserver::new("bar".to_string())),
                            Box::new(EvenObserver::new("baz".to_string())),
                            Box::new(EvenObserver::new("quux".to_string())));
 
    foo.register(bar);
    foo.register(baz);
    foo.register(quux);
 
    foo.run();
}
