# task-cli
[task-cli]https://github.com/ruzul2185/task-cli
**task cli** is a simple command-line task tracker writter in Rust.
It allows you to manage and track tasks efficienly usinga lightweight JSON-based storage system.


---


## Features


- Add new tasks
- Update existing tasks
- Delete tasks
- Mark tasks as **in progress*
- mark tasks as **done**
- List all tasks
- Filter tasks by status:
    - Todo
    - In progress
    - Done


---


## Installation


### Prerequisites


Make sure you have Rust installed:
<https://rust-lang.org/tools/install>


### Build from source


```bash
git clone https://github.com/ruzul2185/task-cli.git
cd task-cli
cargo build --release
```

The binary will be located at:

```bash
target/release/task-cli
```
(Optional + Recommended) Move it into your PATH to use it globally.


## Usage

The CLI stores tasks in a local list.json file.
```bash
task-cli <command> [arguments]
```


## Commands


| Command            | Arguments            | Description                    |
| ------------------ | -------------------- | ------------------------       |
| `add`              | `<description>`      | Add a new task                 |
| `update`           | `<id> <description>` | Update task description        |
| `delete`           | `<id>`               | Delete a task                  |
| `mark-in-progress` | `<id>`               | Mark task as in progress       |
| `mark-done`        | `<id>`               | Mark task as done              |
| `list`             | `_, todo, in-progress, done| in-progress              |


## Examples

Add a new task:
```bash
task-cli add "Description goes here"
```

Update a task;
```bash
task-cli update 1 "Updated description here"
```

Delete a task:
```bash
task-cli delete 1
```

Mark a task a in progress:
```bash
task-cli mark-in-progress 2
```

Mark a task as done:
```bash
task-cli mark-done 2
```

List all tasks:
```bash
task-cli list
```

List only completed tasks:
```bash
task-cli list done
```

## Data Storage
- Tasks are stored locally in list.json file
- File is automatically created on first run
- No database required

## Contributing

Contributions are welcome!
- Fork the repository
- Create a new branch
- Commit your changes
- Open a pull request

Suggested Improvements:
- Better argument parsing
- Improve Error handling
- Preventing race condition for list.json
- Unit tests
