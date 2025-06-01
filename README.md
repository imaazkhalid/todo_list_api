# To-Do List API

A robust and efficient RESTful API for managing a to-do list, meticulously crafted in Rust using the Axum web framework and SQLite for persistent storage. This project demonstrates core backend development principles including CRUD operations, data validation, custom error handling, and asynchronous processing.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
  - [1. Clone the Repository](#1-clone-the-repository)
  - [2. Configure Environment Variables](#2-configure-environment-variables)
  - [3. Build the Project](#3-build-the-project)
  - [4. Run the Application](#4-run-the-application)
- [API Documentation](#api-documentation)
  - [Data Models](#data-models)
    - [Todo Object](#todo-object)
    - [CreateTodo Payload](#createtodo-payload)
    - [UpdateTodo Payload](#updatetodo-payload)
  - [Error Handling](#error-handling)
  - [Endpoints](#endpoints)
    - [Create a To-Do (`POST /todos`)](#create-a-to-do-post-todos)
    - [Get All To-Dos (`GET /todos`)](#get-all-to-dos-get-todos)
    - [Get a Specific To-Do by ID (`GET /todos/{id}`)](#get-a-specific-to-do-by-id-get-todosid)
    - [Update a To-Do by ID (`PUT /todos/{id}`)](#update-a-to-do-by-id-put-todosid)
    - [Delete a To-Do by ID (`DELETE /todos/{id}`)](#delete-a-to-do-by-id-delete-todosid)
- [Database Schema](#database-schema)
- [Logging](#logging)
- [Future Enhancements](#future-enhancements)
- [License](#license)

## Overview

This project provides a complete backend solution for a to-do application. It exposes a set of REST API endpoints to create, read, update, and delete (CRUD) to-do items. The application is built with a focus on modern Rust practices, leveraging the performance and safety features of the language, the ergonomic Axum framework for web handling, and `sqlx` for asynchronous, type-safe SQL interaction with an SQLite database.

The database and its schema are automatically initialized by the application on its first run, ensuring a seamless setup experience for developers and reviewers.

## Features

* **Full CRUD Operations:** Create, retrieve, update, and delete to-do items.
* **Asynchronous Processing:** Built with `tokio` and `async/await` for efficient, non-blocking I/O.
* **Data Validation:** Server-side validation of input data for creating and updating to-dos using the `validator` crate.
* **SQLite Persistence:** Data is reliably stored in an SQLite database file (`todos.db`).
* **Automatic Database Setup:** The database file and schema are created on application startup if they don't exist, simplifying initial setup.
* **Custom Error Handling:** Graceful error responses in a consistent JSON format with appropriate HTTP status codes.
* **Structured Logging:** Utilizes the `tracing` crate for detailed and configurable application logs.
* **JSON API:** All API requests and responses use the JSON data format.
* **Modern Tooling:** Leverages the latest stable versions of Rust, Axum, and other key ecosystem crates.

## Tech Stack

* **Language:** [Rust](https://www.rust-lang.org/)
* **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
* **Asynchronous Runtime:** [Tokio](https://tokio.rs/)
* **Database Interaction:** [SQLx](https://github.com/launchbadge/sqlx)
* **Database:** [SQLite](https://www.sqlite.org/index.html)
* **Serialization/Deserialization:** [Serde](https://serde.rs/)
* **Data Validation:** [Validator](https://crates.io/crates/validator)
* **Unique Identifiers:** [UUID](https://crates.io/crates/uuid)
* **Date & Time:** [Chrono](https://crates.io/crates/chrono)
* **Logging:** [Tracing](https://crates.io/crates/tracing) & `tracing-subscriber`
* **Environment Variables:** [Dotenvy](https://crates.io/crates/dotenvy)

## Prerequisites

Before you begin, ensure you have the following installed on your system:

* **Rust Toolchain:**
    * You can install Rust via [rustup.rs](https://rustup.rs/). A recent stable version is recommended.
        ```bash
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        source $HOME/.cargo/env # Or restart your terminal
        ```
    * Verify your installation: `rustc --version` and `cargo --version`.
* **`curl` or an API Client:** For interacting with and testing the API endpoints (e.g., Postman, Insomnia, or command-line `curl`).

*(SQLite development libraries are generally **not** required as `sqlx` typically bundles the SQLite C source when its `sqlite` feature is enabled).*

## Getting Started

Follow these instructions to set up and run the project locally.

### 1. Clone the Repository

First, clone the repository to your local machine:
```bash
git clone [https://github.com/imaazkhalid/todo_list_api.git](https://github.com/imaazkhalid/todo_list_api.git)
cd your-repository-name
````

### 2\. Configure Environment Variables

The application requires a `.env` file for configuration. An example template (`.env.example`) is provided.

1.  **Copy the example environment file to `.env`:**
    ```bash
    cp .env.example .env
    ```
2.  **Verify the contents of `.env`:**
    The default settings are usually sufficient for local development:
    ```env
    DATABASE_URL=sqlite:todos.db
    RUST_LOG=todo_list_api=debug,tower_http=debug,info
    ```
      * `DATABASE_URL`: Specifies the path to the SQLite database file. `todos.db` will be created in the project's root directory.
      * `RUST_LOG`: Controls the verbosity and scope of logging. `todo_list_api` should match your package name if different.

### 3\. Build the Project

Compile the application using Cargo. This will download dependencies and build the executable.

```bash
cargo build
```

For an optimized release build (recommended for performance testing or "production-like" scenarios):

```bash
cargo build --release
```

### 4\. Run the Application

Once the build is complete, start the API server:

```bash
cargo run
```

If you created a release build, you can run the optimized executable directly:

```bash
./target/release/todo_list_api
```

Upon successful startup, you will see log messages in your terminal, indicating that the server is running and listening for requests, typically on `http://127.0.0.1:3000`. The application will also ensure the database file (`todos.db`) and its schema are created if they don't already exist.

Example startup logs:

```
INFO todo_list_api: Starting server...
INFO todo_list_api: DATABASE_URL from env: sqlite:todos.db
INFO todo_list_api: Attempting to check/create database file at absolute path: "/todo_list_api/todos.db"
INFO todo_list_api: Successfully touched/opened database file via std::fs: "todos.db"
INFO todo_list_api: Connecting to database with sqlx: sqlite:todos.db
INFO todo_list_api: Database pool created. Running schema setup...
INFO todo_list_api: Schema setup complete.
INFO todo_list_api: Listening on 127.0.0.1:3000
```

## API Documentation

The API provides endpoints for managing to-do items. All requests and responses are in JSON format.

  * **UUIDs:** In JSON responses, UUIDs are represented as simple 32-character hexadecimal strings (e.g., `f7c3f1e97a824faca5ac5e9f597f1db3`). When providing a UUID as a path parameter (e.g., for `GET /todos/{id}`), standard hyphenated UUID format is also accepted.
  * **Timestamps:** All timestamps are in UTC and formatted according to RFC3339 / ISO 8601 (e.g., `2025-06-01T13:30:52.123456Z`).

### Data Models

#### Todo Object

Represents a to-do item as returned by the API.

| Field         | Type                      | Description                                         |
|---------------|---------------------------|-----------------------------------------------------|
| `id`          | String (UUID)             | Unique identifier (32-char hex string, no hyphens). |
| `title`       | String                    | The title or name of the to-do item.                |
| `description` | String / `null` (Optional) | A detailed description of the to-do task.         |
| `completed`   | Boolean                   | Indicates if the to-do item is marked as complete.  |
| `created_at`  | String (RFC3339 Timestamp)| Timestamp of when the item was created (UTC).       |
| `updated_at`  | String (RFC3339 Timestamp)| Timestamp of when the item was last updated (UTC).  |

#### CreateTodo Payload

JSON payload for creating a new to-do item.

| Field         | Type                      | Required | Validation / Notes                                     |
|---------------|---------------------------|----------|--------------------------------------------------------|
| `title`       | String                    | Yes      | Must not be empty (minimum length of 1 character).     |
| `description` | String / `null` (Optional) | No       | A detailed description. Can be `null` or omitted.      |

#### UpdateTodo Payload

JSON payload for updating an existing to-do item. All fields are optional.

  * If a field is provided, its value will be updated.
  * If a field is omitted from the payload, its current value in the database will be retained.
  * Providing `null` for `description` will set the description to `NULL` in the database.

| Field         | Type                        | Required | Validation / Notes                                         |
|---------------|-----------------------------|----------|------------------------------------------------------------|
| `title`       | String (Optional)           | No       | If provided, must not be empty (min length 1).             |
| `description` | String / `null` (Optional)  | No       | New description. `null` sets the database field to `NULL`. |
| `completed`   | Boolean (Optional)          | No       | New completion status (`true` or `false`).                 |

### Error Handling

The API returns errors in a consistent JSON format:

```json
{
  "error": "A descriptive error message relevant to the issue."
}
```

Common HTTP status codes used for errors:

  * `400 Bad Request`: For request validation errors (e.g., missing required fields, invalid data format) or malformed JSON.
  * `404 Not Found`: If a requested resource (e.g., a specific to-do item via its ID) does not exist.
  * `500 Internal Server Error`: For unexpected issues on the server-side (e.g., database connectivity problems not otherwise handled). Detailed errors are logged server-side.

### Endpoints

The base URL for the API when running locally is `http://localhost:3000`.

-----

#### Create a To-Do (`POST /todos`)

Creates a new to-do item.

  * **Method:** `POST`
  * **Path:** `/todos`
  * **Headers:**
      * `Content-Type: application/json`
  * **Request Body:** `CreateTodo` payload (see [Data Models](https://www.google.com/search?q=%23createtodo-payload))
    ```json
    {
      "title": "Plan Weekend Trip",
      "description": "Research destinations and book accommodation."
    }
    ```
  * **Success Response (201 Created):** Returns the newly created `Todo` object.
    ```json
    {
      "id": "a1b2c3d4e5f67890a1b2c3d4e5f67890",
      "title": "Plan Weekend Trip",
      "description": "Research destinations and book accommodation.",
      "completed": false,
      "created_at": "2025-06-01T12:00:00.123456Z",
      "updated_at": "2025-06-01T12:00:00.123456Z"
    }
    ```
  * **Error Responses:**
      * `400 Bad Request`: If `title` is missing or empty.
  * **`curl` Example:**
    ```bash
    curl -X POST http://localhost:3000/todos \
    -H "Content-Type: application/json" \
    -d '{
      "title": "Grocery Shopping",
      "description": "Milk, eggs, bread, and Rust learning materials."
    }'
    ```

-----

#### Get All To-Dos (`GET /todos`)

Retrieves a list of all to-do items, ordered by creation date (newest first).

  * **Method:** `GET`
  * **Path:** `/todos`
  * **Request Body:** None
  * **Success Response (200 OK):** An array of `Todo` objects. Returns an empty array `[]` if no to-dos exist.
    ```json
    [
      {
        "id": "a1b2c3d4e5f67890a1b2c3d4e5f67890",
        "title": "Grocery Shopping",
        "description": "Milk, eggs, bread, and Rust learning materials.",
        "completed": false,
        "created_at": "2025-06-01T12:00:00.123456Z",
        "updated_at": "2025-06-01T12:00:00.123456Z"
      },
      {
        "id": "f9e8d7c6b5a43210f9e8d7c6b5a43210",
        "title": "Finish Rust Project",
        "description": "Complete the To-Do API and write README.",
        "completed": true,
        "created_at": "2025-05-31T18:30:00.987654Z",
        "updated_at": "2025-06-01T09:15:00.543210Z"
      }
    ]
    ```
  * **`curl` Example:**
    ```bash
    curl http://localhost:3000/todos
    ```

-----

#### Get a Specific To-Do by ID (`GET /todos/{id}`)

Retrieves a single to-do item identified by its UUID.

  * **Method:** `GET`
  * **Path:** `/todos/{id}`
      * `{id}`: The UUID of the to-do item (e.g., `a1b2c3d4e5f67890a1b2c3d4e5f67890`). Hyphenated or simple format accepted in the path.
  * **Request Body:** None
  * **Success Response (200 OK):** The requested `Todo` object.
  * **Error Responses:**
      * `404 Not Found`: If no to-do item with the specified ID exists.
      * `400 Bad Request` / `404 Not Found`: If the `{id}` path parameter is not a valid UUID format (Axum's path extractor may lead to a 404 if the route pattern isn't fully matched due to parsing failure).
  * **`curl` Example:**
    ```bash
    curl http://localhost:3000/todos/a1b2c3d4e5f67890a1b2c3d4e5f67890
    ```

-----

#### Update a To-Do by ID (`PUT /todos/{id}`)

Updates an existing to-do item. Only the fields provided in the JSON request body are modified.

  * **Method:** `PUT`
  * **Path:** `/todos/{id}`
      * `{id}`: The UUID of the to-do item to update.
  * **Headers:**
      * `Content-Type: application/json`
  * **Request Body:** `UpdateTodo` payload (see [Data Models](https://www.google.com/search?q=%23updatetodo-payload)).
    Example updating title and completion status:
    ```json
    {
      "title": "Grocery Shopping (Urgent)",
      "completed": true
    }
    ```
    Example clearing the description:
    ```json
    {
      "description": null
    }
    ```
  * **Success Response (200 OK):** The fully updated `Todo` object.
  * **Error Responses:**
      * `400 Bad Request`: If validation fails for any provided field (e.g., `title` is an empty string).
      * `404 Not Found`: If no to-do item with the specified ID exists.
  * **`curl` Example:**
    ```bash
    curl -X PUT http://localhost:3000/todos/a1b2c3d4e5f67890a1b2c3d4e5f67890 \
    -H "Content-Type: application/json" \
    -d '{
      "completed": true,
      "description": "All items purchased."
    }'
    ```

-----

#### Delete a To-Do by ID (`DELETE /todos/{id}`)

Deletes a specific to-do item identified by its UUID.

  * **Method:** `DELETE`
  * **Path:** `/todos/{id}`
      * `{id}`: The UUID of the to-do item to delete.
  * **Request Body:** None
  * **Success Response (204 No Content):** No response body. The HTTP status code confirms successful deletion.
  * **Error Responses:**
      * `404 Not Found`: If no to-do item with the specified ID exists.
  * **`curl` Example:**
    ```bash
    curl -X DELETE http://localhost:3000/todos/a1b2c3d4e5f67890a1b2c3d4e5f67890
    ```
    To view the status code with `curl`, use the `-i` flag:
    ```bash
    curl -i -X DELETE http://localhost:3000/todos/a1b2c3d4e5f67890a1b2c3d4e5f67890
    ```

-----

## Database Schema

The application utilizes a single SQLite table named `todos` to store the to-do items. The schema is defined as follows and created automatically by the application if it doesn't exist:

```sql
CREATE TABLE IF NOT EXISTS todos (
    id TEXT PRIMARY KEY NOT NULL, -- Stores UUID as a string
    title TEXT NOT NULL,
    description TEXT,             -- Nullable, stores detailed text
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- Stores as TEXT in RFC3339 format
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP  -- Stores as TEXT in RFC3339 format
);
```

## Logging

Structured logging is implemented via the `tracing` crate and `tracing-subscriber`. Log output behavior (level, format) can be configured using the `RUST_LOG` environment variable specified in the `.env` file. The default setting `RUST_LOG=todo_list_api=debug,tower_http=debug,info` provides:

  * `debug` level logs for this specific application (`todo_list_api`).
  * `debug` level logs for `tower_http` (Axum's underlying HTTP library, useful for seeing request/response traces).
  * `info` level as a fallback for other crates.

Logs are output to the standard console where the application is running.

## Future Enhancements

This project serves as a comprehensive demonstration of a basic CRUD API. Potential future enhancements could include:

  * **Pagination & Filtering:** Implement pagination for the `GET /todos` endpoint to handle large datasets efficiently, and add query parameters for filtering (e.g., by completion status) and sorting.
  * **User Authentication & Authorization:** Integrate user accounts and JWT-based authentication to enable multi-user support and private to-do lists.
  * **More Granular Validation:** Implement more complex or conditional validation rules.
  * **Automated Testing:** Develop a suite of unit tests for business logic and integration tests for API endpoints using Rust's testing framework and crates like `reqwest` or `hyper`.
  * **Containerization:** Provide a `Dockerfile` to build and run the application in a Docker container for easier deployment and portability.
  * **OpenAPI/Swagger Documentation:** Use crates like `utoipa` to generate interactive OpenAPI (Swagger) documentation directly from code comments and structs.
  * **Configuration Management:** Enhance configuration beyond `.env` for aspects like server port, database path, etc., perhaps using a configuration file or more advanced environment variable parsing.
