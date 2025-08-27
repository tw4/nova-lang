# Nova CLI Application

A command-line task manager application demonstrating advanced Nova features and CLI development patterns.

## Files

- `main.nova` - Main application with TaskManager class and command parsing

## Features Demonstrated

- **Advanced Classes**: Complex class with multiple methods and state management
- **Object-Oriented Design**: Encapsulation of task management logic
- **Command Parsing**: String manipulation and argument parsing
- **Control Structures**: Switch statements, loops, conditionals
- **Data Structures**: Arrays, objects, and complex data manipulation
- **File I/O Simulation**: Loading and saving data (simulated)
- **Error Handling**: Validation and user feedback
- **String Operations**: Trimming, splitting, joining

## Running

To run this CLI application:

```bash
nova main.nova
```

## Commands

- `add <description>` - Add a new task
- `list` - List all tasks with status
- `complete <id>` - Mark a task as completed
- `delete <id>` - Remove a task
- `help` - Show available commands
- `quit` - Exit the application

## Architecture

```
TaskManager
├── constructor() - Initialize with empty task list
├── addTask(description) - Create and store new task
├── listTasks() - Display all tasks with formatting
├── completeTask(id) - Mark task as done
├── deleteTask(id) - Remove task from list
├── loadTasks() - Load tasks from storage
├── saveTasks() - Persist tasks to storage
└── showHelp() - Display command help

Utilities
├── parseCommand(input) - Parse user command and arguments
└── main() - Application entry point and command loop
```

## Data Model

```nova
task = {
    id: number,
    description: string,
    completed: boolean,
    createdAt: Date
}
```

## Next Steps

- Add task priorities and categories
- Implement due dates and reminders
- Add task search and filtering
- Create a configuration system
- Add data export/import functionality
- Build a web interface version