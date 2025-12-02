
# Todo app

> !Note it creates a ~/.todo file in your home dir

#### Build

`cargo build` to build the application
`cargo install --path . ` installs it in ~/.cargo/bin

---

#### Help

```
use case:
    todo [operation] [argument]

operations:
    --list   Lists all Tasks in to-do (default operation, no args)
    --add    Adds task to to-do (requires 1 argument)
    --remove Removes a task with the provided task ID (requires 1 argument)
    --help   Prints help message

examples:
    todo --list
        Lists all the tasks

    todo --add "Touch grass"
        Adds the task

    todo --remove 1
        Removes task which has ID 1
```
