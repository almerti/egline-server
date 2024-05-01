# [Egline](https://github.com/almerti/Egline) server

This is a server application developed using Rust, leveraging the [Rocket web framework](https://rocket.rs/) and [SeaORM](https://www.sea-ql.org/SeaORM/) for database operations, specifically with [PostgreSQL](https://www.postgresql.org/). 

## How to start?

To run the server successfully, you will need a database. We used PostgreSQL, although you can use MySQL or SQLite (as required by SeaORM).

### Database

To set up a PostgreSQL database for your server, you can either create a local database or use a Docker container. Using a Docker container is recommended, especially with an Alpine Linux-based image, as it's lightweight and efficient.

### Server

To set up and run your project using SeaORM, follow these steps:
1. Clone or download the project repository to your local machine.
2. Open the .env file in the root of your project and update the DATABASE_URL variable to match your database connection details. The format should be protocol://username:password@host/database. Make same changes in setup file.
3. In the root directory of your project, execute the command to apply all pending migrations and create the necessary tables in your database: ```sea-orm-cli migrate up```
4. If you've made changes to the migration files, you need to regenerate the entity files. Run ```sea-orm-cli generate entities -o src/entities --with-serde both``` to update the entity files in the src/entities directory. This step is crucial for ensuring that your entity definitions match the current state of your database schema.
5. After generating the entities, review and update any route methods in the src/routes folder to reflect any changes in your entity definitions or business logic.
6. Finally, start your application by running ```cargo run``` in the terminal from the root directory of your project. This command compiles and runs your Rust application, making it accessible via the configured port.

Remember, the sea-orm-cli tool is essential for managing database migrations and generating entity files. Ensure it's installed by running ```cargo install sea-orm-cli``` if you haven't already.