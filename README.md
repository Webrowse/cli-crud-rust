# notez - A Zero-Dependency Rust CLI To-Do Manager

`notez` is a command-line task manager built in Rust without any external dependencies. It allows users to add, view, complete, and delete tasks using simple commands, storing data locally in a `Tasks.txt` file.

## Features

* Zero dependencies (no `clap`, `serde`, or other crates)
* Simple text-based task storage
* Cross-platform compatible (as long as Rust is installed)

## Getting Started

You must have Rust installed. Visit [https://rustup.rs](https://rustup.rs) for setup instructions.

### Build the Application

```bash
cargo build --release
```

The compiled binary will be located at `target/release/notez`.

## Usage

All task data is stored in `Tasks.txt` in the same directory as the binary.

### Add Task

```bash
./notez add "Buy groceries"
```

### List Tasks

```bash
./notez list
```

Sample Output:

```
1   [ ] Buy groceries
2   [X] Walk the dog
```

### Complete or Incomplete Toggle

```bash
./notez complete 1
```

Toggles task 1 between completed `[X]` and incomplete `[ ]`.

### Delete Task

```bash
./notez delete 1
```

Deletes the task with the given number.

## Limitations

* Task storage is plain text, no structured format like JSON or CSV
* File is overwritten each time a task is completed or deleted

## License

This project is licensed under the MIT License.

---

A pure, zero-dependency CLI utility in Rust to keep your task list clean and manageable.
