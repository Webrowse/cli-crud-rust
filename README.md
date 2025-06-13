Rust CLI To-Do Application

This command-line application, developed in Rust, facilitates fundamental task management: adding, listing, modifying status, and deleting tasks directly from the terminal.
Getting Started

Deployment requires Rust installation, with instructions available at rust-lang.org. To compile, save the source as main.rs, navigate to its directory, and execute rustc main.rs. This generates an executable file (e.g., main or main.exe).
Operational Instructions

Invoke the compiled binary with a command and its arguments. All task data persists in Tasks.txt within the execution directory.
Available Commands:

    Add Task: ./main add <description>. New tasks are initialized as incomplete [ ].

        Example: ./main add Acquire supplies

    List Tasks: ./main list. Displays all recorded tasks with numerical indexing.

        Example: ./main list (Output: 1. [ ] Acquire supplies)

    Modify Status: ./main complete <number>. Toggles task completion status [ ] ↔ [X].

        Example: ./main complete 1

    Delete Task: ./main delete <number>. Permanently removes a task from the list.

        Example: ./main delete 2

Capabilities & Limitations

This foundational CLI tool demonstrates Rust CLI and file handling. It stores tasks in Tasks.txt and incorporates basic error handling. Future enhancements may include an interactive mode, refined error messaging, extended features (e.g., prioritization), and enhanced robustness.
