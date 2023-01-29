enum Command {
    Todo(String),
    Done(usize),
    Purge,
    Quit,
}

impl Command {
    fn prompt() -> Self {
        loop {
            let line = ftkit::read_line();

            if line.is_empty() || line.trim() == "QUIT" {
                break Self::Quit;
            } else if let Some(task) = line.strip_prefix("TODO ") {
                break Self::Todo(task.trim().to_string());
            } else if let Some(index) = line.strip_prefix("DONE ") {
                if let Ok(index) = index.trim().parse() {
                    break Self::Done(index);
                }
            } else if line.trim() == "PURGE" {
                break Self::Purge;
            }
        }
    }
}

struct TodoList {
    todos: Vec<String>,
    dones: Vec<String>,
}

impl TodoList {
    fn new() -> Self {
        Self {
            todos: Vec::new(),
            dones: Vec::new(),
        }
    }

    fn display(&self) {
        if !self.todos.is_empty() || !self.dones.is_empty() {
            println!();
        }
        for i in 0..self.todos.len() {
            println!("   \x1B[96m{:>3} \x1B[90m[ ] \x1B[0m{}", i, &self.todos[i]);
        }
        for done in &self.dones {
            println!("       \x1B[90m[x] {done}\x1B[0m");
        }
    }

    fn add(&mut self, todo: String) {
        self.todos.push(todo);
    }

    fn done(&mut self, index: usize) {
        if index >= self.todos.len() {
            return;
        }

        let s = self.todos.remove(index);
        self.dones.push(s);
    }

    fn purge(&mut self) {
        self.dones.clear();
    }
}

fn main() {
    let mut todolist = TodoList::new();

    loop {
        todolist.display();
        println!();
        match Command::prompt() {
            Command::Todo(todo) => todolist.add(todo),
            Command::Done(index) => todolist.done(index),
            Command::Purge => todolist.purge(),
            Command::Quit => break,
        }
    }
}
