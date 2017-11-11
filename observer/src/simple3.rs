trait Subject<T: Clone> {
    fn notify_observers(&self, &T);
    fn register_observer(&mut self, Box<Observer<T>>) -> usize;
    fn unregister_observer(&mut self, usize);
}


trait Observer<T: Clone> {
    fn on_notify(&self, &T);
}


#[derive(Debug, Clone)]
struct EventObject(usize);


struct SubjectX {
    observers: Vec<(bool, Box<Observer<EventObject>>)>,
}


impl SubjectX {
    fn new() -> SubjectX
    {
        SubjectX {
            observers: Vec::new(),
        }
    }
}


impl Subject<EventObject> for SubjectX {
    fn notify_observers(&self, e: &EventObject)
    {
        for observer in self.observers.iter() {
            if observer.0 {
                observer.1.on_notify(e);
            }
        }
    }

    fn register_observer(&mut self, o: Box<Observer<EventObject>>) -> usize
    {
        self.observers.push((true, o));
        self.observers.len() - 1
    }

    fn unregister_observer(&mut self, i: usize)
    {
        self.observers[i].0 = false
    }
}


struct ObserverX(usize);
impl Observer<EventObject> for ObserverX {
    fn on_notify(&self, e: &EventObject)
    {
        println!("ObserverX {} Get {:?}", self.0, e);
    }
}


fn main()
{
    let mut subject = SubjectX::new();
    subject.register_observer(Box::new(ObserverX(1)));
    subject.register_observer(Box::new(ObserverX(2)));
    subject.register_observer(Box::new(ObserverX(3)));

    subject.notify_observers(&EventObject(100));
    subject.notify_observers(&EventObject(20));
}