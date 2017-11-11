struct Interpreter;

impl Interpreter {
    fn parse_and_execute(s: String)
    {
        let mut s = s.clone();
        if let Some(i) = s.find(' ') {
            let word = s.split_off(i);
            let times = s.parse::<usize>().unwrap();
            for _ in 0..times {
                print!("{} ", word);
            }
            println!("");
        }
    }
}


fn main()
{
    Interpreter::parse_and_execute("10 hey guys !".to_string());
}