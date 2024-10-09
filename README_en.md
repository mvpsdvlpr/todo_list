
# Task Manager in Rust

This is a basic console project for managing tasks (to-do list) in Rust. It allows you to add, list, update, and delete tasks while applying various fundamental language concepts such as structs, enums, and more.

## Features

- **Add a new task**: Users can add tasks with a title, description, and an initial status.
- **List tasks**: Displays all tasks along with their current status.
- **Update a task’s status**: Allows marking a task as `Pending`, `In Progress`, or `Completed`.
- **Delete a task**: Removes a task from the list.
- **Optional**: Save and load tasks from a file.

## Applied Concepts

This project uses:
- **Variables, Structs, and Enums**: To represent tasks and their status.
- **Functions**: To handle CRUD (Create, Read, Update, Delete) operations on tasks.
- **Option and match**: To safely handle non-existent tasks.
- **Ownership and Borrowing**: To correctly manage task data and concurrent access.

## Running the Project

### Prerequisites

- Have Rust installed. If you don’t have it, install it by following the instructions on [rust-lang.org](https://www.rust-lang.org/).

### Installation

1. Clone this repository to your local machine:
   ```bash
   git clone https://github.com/your-username/task-manager-rust.git
   cd task-manager-rust
   ```

2. Compile and run the program:
   ```bash
   cargo run
   ```

## Usage Example

### Adding a Task
When the program starts, the user can add a task by providing a title and description.

```rust
let task = Task {
    title: String::from("Learn Rust"),
    description: String::from("Review basic Rust concepts"),
    status: Status::Pending,
};
```

### Listing Tasks
The program displays all stored tasks and their current status.

```bash
Task: "Learn Rust", Status: Pending
```

### Updating a Task’s Status
The user can change a task’s status to `In Progress` or `Completed` based on progress.

### Deleting a Task
The program allows easy removal of tasks from the list.

## Future Improvements

- **User Interaction**: Implement a menu system allowing users to perform all actions from the console.
- **Data Persistence**: Save and load tasks from a file to avoid losing data when the program closes.
- **User Interface**: Create a graphical interface using tools like `Yew` or integrate with a WebAssembly frontend.

## Contributions

Contributions are welcome. If you have ideas or improvements, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. For more details, see the [LICENSE](LICENSE) file.
