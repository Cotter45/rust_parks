# Hello Parks API 🌲

Hello Parks API is a Rust-based web application that provides endpoints to access detailed information about various parks and states. Utilizing Actix-Web, a powerful, pragmatic, and extremely fast web framework for Rust, this API ensures high performance and reliability.

## Features 🌟

- Get All Parks: Retrieve details of all parks, including id, name, location, and description.
- Get Park by ID: Access specific details of a park using its unique ID.
- Search Parks: Perform a search to find parks based on names or descriptions.
- Get All States: Access details of all states including id, state name, total parks, etc.
- Get State by ID: Access specific details of a state using its unique ID.
- Search States: Perform a search to find states by name.

## Endpoints 🚀

### Parks

- GET /parks: Retrieves all parks
- GET /parks/{id}: Retrieves a specific park by ID
- GET /parks/search/{query}: Searches for parks matching the query

### States

- GET /states: Retrieves all states
- GET /states/{id}: Retrieves a specific state by ID
- GET /states/search/{query}: Searches for states matching the query

## Setup and Running 🛠️

### Prerequisites

- Rust Programming Language
- Cargo (Rust's package manager)

### Running the Application

- Clone the repository to your local machine.

```bash
git clone https://github.com/your-username/hello-parks-api.git
```

- Navigate to the project directory.

```bash
cd rust_parks
```

- Build and run the application.

```bash
cargo run
```

- Access the application at: http://127.0.0.1:8080

### Docker Deployment 🐳

This application comes with a Dockerfile and a Docker Compose configuration for easy deployment. Follow the instructions below to deploy using Docker:

- Build the Docker image.

```
docker-compose up --build -d
```

Now, the application should be accessible at http://127.0.0.1:8080

## Contributing 🤝

Contributions are welcome! Feel free to submit issues or pull requests to improve the project.

## License 📄

This project is licensed under the MIT License - see the LICENSE file for details.
