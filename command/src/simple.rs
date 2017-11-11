trait Command<T> {
    fn execute(&self, &mut T);
    fn undo(&self, &mut T);
}


struct Invoker<'a, T: 'a> {
    commands: Vec<Box<Command<T> + 'a>>,
    target: &'a mut T,
    current_index: usize,
}


impl<'a, T> Invoker<'a, T> {
    fn new(t: &'a mut T) -> Self {
        Invoker {
            commands: Vec::new(),
            target: t,
            current_index: 0,
        }
    }


    fn target(&self) -> &T {
        self.target
    }

    fn append_command<U: Command<T> + 'a>(&mut self, c: U) {
        self.commands.push(Box::new(c));
    }

    fn execute_command(&mut self) {
        if self.commands.len() <= self.current_index {
            // Nothing to do.
            return;
        }

        let c = &*self.commands[self.current_index];
        let t = &mut *self.target;
        c.execute(t);

        self.current_index += 1;
    }

    fn execute_all_commands(&mut self) {
        for _ in self.current_index..self.commands.len() {
            self.execute_command();
        }
    }

    fn undo(&mut self) {
        if 0 == self.current_index {
            return;
        }

        self.current_index -= 1;

        let c = &*self.commands[self.current_index];
        let t = &mut *self.target;
        c.undo(t);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}


impl Robot {
    fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            dx: 0,
            dy: 1,
        }
    }

    fn move_forward(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    fn set_direction(&mut self, d: (i32, i32)) {
        self.dx = d.0;
        self.dy = d.1;
    }

    fn get_direction(&self) -> (i32, i32) {
        (self.dx, self.dy)
    }
}


struct CommandMoveForward;
impl Command<Robot> for CommandMoveForward {
    fn execute(&self, r: &mut Robot) {
        r.move_forward();
    }

    fn undo(&self, r: &mut Robot) {
        let c1 = CommandTurnRight;
        c1.execute(r);
        c1.execute(r);
        self.execute(r);
        c1.execute(r);
        c1.execute(r);
    }
}


struct CommandTurnRight;
impl Command<Robot> for CommandTurnRight {
    fn execute(&self, r: &mut Robot) {
        let (dx, dy) = r.get_direction();
        r.set_direction((dy, -dx));
    }

    fn undo(&self, r: &mut Robot) {
        let c = CommandTurnLeft;
        c.execute(r);
    }
}


struct CommandTurnLeft;
impl Command<Robot> for CommandTurnLeft {
    fn execute(&self, r: &mut Robot) {
        let (dx, dy) = r.get_direction();
        r.set_direction((-dy, dx));
    }

    fn undo(&self, r: &mut Robot) {
        let c = CommandTurnRight;
        c.execute(r);
    }
}


fn main() {
    let mut r = Robot::new();

    let mut invoker = Invoker::new(&mut r);
    assert_eq!(*invoker.target(), Robot { x: 0, y: 0, dx: 0, dy: 1, });

    invoker.append_command(CommandTurnRight);
    invoker.append_command(CommandTurnLeft);
    invoker.append_command(CommandMoveForward);

    invoker.execute_all_commands();
    assert_eq!(*invoker.target(), Robot { x: 0, y: 1, dx: 0, dy: 1, });

    invoker.undo();
    assert_eq!(*invoker.target(), Robot { x: 0, y: 0, dx: 0, dy: 1, });

    invoker.undo();
    assert_eq!(*invoker.target(), Robot { x: 0, y: 0, dx: 1, dy: 0, });
}