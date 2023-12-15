# Rush Yale API

A Rust API to manage a cappella "Rush" at Yale University that conforms to the OpenAPI
v3 specification. The application uses MySQL as its DBMS, _SeaORM_ as its Rust ORM, and
_actix-web_ as its Rust web framework.

## Running the Application

1. Create a table in your MySQL database

2. Run the included `rush-yale-schema.sql` file on that table.

3. Create a `.env` file according to the `.env.example` file with the first step's table name.

4. Create two files `cert.pem` and `key.pem` with a TLS certificate for the domain
   on which you will be running the application (e.g., `localhost`) and store them in the
   root of the `backend` directory / the Rust crate.

5. Run the application with `cargo run` from the `backend` directory / Rust crate.

## Querying the API

The API specification details the exact parameters of each API endpoint, so you can
import the specification into SmartBear's [SwaggerEditor](https://editor-next.swagger.io/)
and use that interface to query the API. Once you have a better understanding of the API,
you can turn to tools like `curl` to query the API.
