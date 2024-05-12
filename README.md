# [Egline](https://github.com/almerti/Egline) server

## Overview

This is a server application developed using Rust, leveraging the [Rocket web framework](https://rocket.rs/) and [SeaORM](https://www.sea-ql.org/SeaORM/) for database operations, specifically with [PostgreSQL](https://www.postgresql.org/). 

## Prerequisites

- Rust: Ensure you have Rust installed on your system. You can download it from the official [Rust website](https://www.rust-lang.org/tools/install).
- PostgreSQL: The Egline server uses PostgreSQL as the database. You can either set up a local PostgreSQL instance or use a Docker container.

## Database Setup

### Using Docker (Recommended)

1. Install [Docker](https://www.docker.com/get-started/) on your system.
2. From the root directory of the project, run the appropriate script to initialize the database:
 - Windows: db_init.bat
 - Linux: ./db_init.sh

This script will create a Docker container with a PostgreSQL database.

### Manual Setup

1. Install PostgreSQL on your system.
2. Create a new database for the Egline server.
3. Update the DATABASE_URL variable in the .env file to match your database connection details.    

## Running the Server

1. Clone or download the project repository to your local machine.
2. Open the .env file in the root of your project and update the DATABASE_URL variable to match your database connection details. The format should be protocol://username:password@host/database.
3. In the root directory of your project, apply all pending migrations and create the necessary tables in your database: ```sea-orm-cli migrate up```
4. Start your application by running: ```cargo run```
This command compiles and runs your Rust application, making it accessible via the configured port.
5. If you've made changes to the migration files, regenerate the entity files:```sea-orm-cli generate entities -o src/entities --with-serde both```
This step is crucial for ensuring that your entity definitions match the current state of your database schema.
6. Review and update any route methods in the src/routes folder to reflect any changes in your entity definitions or business logic.

The order has been updated to first run the server, then regenerate the entity files and update the route methods if needed. This ensures the server is up and running before making any changes to the codebase.

> [!NOTE]
> Remember, the sea-orm-cli tool is essential for managing database migrations and generating entity files. Ensure it's installed by running ```cargo install sea-orm-cli``` if you haven't already.

## Dependencies
The Egline server uses the following dependencies:

- [Rocket](https://rocket.rs/) - A web framework for Rust
- [SeaORM](https://www.sea-ql.org/SeaORM/) - An asynchronous database ORM for Rust

## Contributing
If you would like to contribute to the Egline server, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and commit them with descriptive commit messages.
4. Push your changes to your forked repository.
5. Submit a pull request to the main repository.

## License
Egline server is released under the [GNU GPL](LICENSE).

