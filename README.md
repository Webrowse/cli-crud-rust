# bt - A Zero-Dependency Rust CLI To-Do App

`bt` is a command-line task manager built in Rust. It allows users to add, view, complete, and delete tasks using simple commands, storing data locally in a `Tasks.txt` file.

---

##  Installation

Ensure Rust is installed. You can install Rust using:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository and build the binary:

```bash
git clone https://github.com/Webrowse/cli-crud-rust.git
cd cli-crud-rust
cargo build --release
```

This will generate an optimized executable at `target/release/bt`.

---

##  Usage

All commands are executed using the binary:

```bash
./bt <command> [arguments]
```

All tasks are stored in a local file named `Tasks.txt`.

### 1. Add a Task

Add a new task marked as incomplete.

```bash
./bt add "Buy groceries"
```

> Output example:  
> `File updated successfully: Tasks.txt`  
> `1. [ ] Buy groceries`

---

### 2. List All Tasks

Display all tasks with numerical indices.

```bash
./bt list
```

> Output:
```
1   [ ] Buy groceries
2   [X] Call mom
```

---

### 3. Mark a Task as Complete/Incomplete

Toggle a task's status using its number.

```bash
./bt complete 2
```

> Output:  
> `Task 2 marked as incomplete`

---

### 4. Delete a Task

Remove a task permanently.

```bash
./bt delete 1
```

> Output:  
> `Deleted 1: [ ] Buy groceries`

---

##  File Storage

All tasks are saved in `Tasks.txt` (plain text) located in the same directory as the binary. Ensure you have write permissions for this file.

---

##  Features

- No external dependencies
- Small binary size
- Pure standard library code
- Robust input validation
- Includes unit tests for core functionality

---

##  Notes

- Task indices start from `1` (human-friendly).
- Empty or invalid inputs are gracefully handled.
- Errors are displayed with clear messages.
- This project is a clean foundation for exploring more advanced CLI applications.

---
