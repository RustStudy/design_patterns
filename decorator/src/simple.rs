trait Component {
    fn do_something(&self);
}


trait Decorator: Component {
    fn do_something_more(&self);
}


struct BaseObject(usize);
impl Component for BaseObject {
    fn do_something(&self)
    {
        println!("something: {}", self.0);
    }
}


struct DecoratedObject {
    base: BaseObject,
    more_value: usize,
}

impl Component for DecoratedObject {
    fn do_something(&self)
    {
        self.base.do_something()
    }
}

impl Decorator for DecoratedObject {
    fn do_something_more(&self)
    {
        println!("something more: {}", self.more_value);
    }
}

fn process(c: &Component)
{
    c.do_something()
}

fn main()
{
    let obj = BaseObject(100);
    process(&obj);

    let dobj = DecoratedObject {
        base: obj,
        more_value: 999,
    };
    process(&dobj);
    dobj.do_something_more();
}