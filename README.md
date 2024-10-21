# Jira TUI Client

A terminal-based user interface (TUI) for interacting with Jira, built in Rust for fun and learning.

## Features

- View current sprint tasks
- Display task details
- Responsive terminal UI

## Architecture

This application follows a hybrid architecture, combining elements of component-based design and Flux architecture. It consists of three main loops:

1. UI Loop
2. State Store Loop
3. API Server Loop

### Key Concepts

- **State**: The central source of truth for the application, managed by the State Store.
- **Actions**: Events that trigger state changes or API calls.
- **Components**: UI elements that render based on the current state and dispatch actions.
- **Middleware**: Intercepts actions to perform side effects (e.g., API calls) before updating the state.

### Main Loops

#### UI Loop (`src/ui/ui_loop.rs`)

- Handles user input and rendering
- Subscribes to state updates
- Dispatches actions based on user interactions

#### State Store Loop (`src/state/state_store.rs`)

- Manages the application state
- Processes actions to update the state
- Broadcasts state changes to subscribers (e.g., UI)

#### API Server Loop (`src/state/server.rs`)

- Handles API-related actions
- Performs asynchronous API calls
- Dispatches new actions based on API responses

### Data Flow

1. User interacts with the UI
2. UI component dispatches an action
3. Middleware intercepts the action (if necessary)
4. State Store processes the action and updates the state
5. UI re-renders based on the new state

## TODO
- Implement release and build process
- Implement create/edit functionalities for Jira tasks
- Improve error handling and user feedback
- Add user authentication and configuration
