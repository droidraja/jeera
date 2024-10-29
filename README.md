# Jira TUI Client

A terminal-based user interface (TUI) for interacting with Jira, built in Rust for fun and learning.

## Features

- View current sprint tasks
- Display task details
- Transition task status 



## Manual Configuration (Can be setup using the tui when starting up)

Jeera CLI requires a configuration file to run. Here's how to set it up:

1. Create a directory named `.jeera` in your home directory:
   - On Linux/macOS: `mkdir ~/.jeera`
   - On Windows: `mkdir %USERPROFILE%\.jeera`

2. Create a configuration file named `config.json` in the `.jeera` directory:
   `~/.jeera/config.json`

   This location is consistent across operating systems (Linux, macOS, and Windows).

3. The `config.json` file must have the following structure:

   ```json
   {
     "email": "your-email@example.com",
     "api_token": "your-jira-api-token",
     "host": "your-jira-host.atlassian.net"
   }
   ```


## Architecture

This application follows a hybrid architecture, combining elements of component-based design and Flux architecture. It consists of three main loops:

1. UI Loop
2. State Store Loop
3. API Server Loop

### Key Concepts

- **State**: The central source of truth for the application, managed by the State Store.
- **Actions**: Events that trigger state changes or API calls.
- **Components**: UI elements that render based on the current state and dispatch actions.
- **Middleware**: Intercepts actions to perform side effects, in our case API calls, before updating the state.

### Main Loops

#### UI Loop (`src/ui/ui_loop.rs`)

- Handles user input and rendering
- Subscribes to state updates, user events (Keyboard and mouse - not used yet)
- Dispatches actions based on user interactions

#### State Store Loop (`src/state/state_store.rs`)

- Manages the application state
- Processes actions to update the state
- Dispatches state changes to the UI

#### API Server Loop (`src/state/server.rs`)

- Acts as a Middleware and intercepts actions dispatched by UI 
- Proxies unhandled Actions to the state store
- Handles API-related actions
- Performs asynchronous API calls
- Dispatches new actions based on API responses to the state store

### Data Flow

1. User interacts with the UI
2. UI component dispatches an action
3. Middleware intercepts the action (if necessary)
4. State Store processes the action and updates the state
5. UI re-renders based on the new state

## TODO

- [x] Implement release and build process
- [ ] Implement create/edit functionalities for Jira tasks
- [ ] Improve error handling and user feedback
- [x] Add user authentication and configuration through TUI
