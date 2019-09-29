# TERM-PROJECT: KennUWare

An ERP system developed using Rust and React, with a PostgreSQL database.

## Team Inventory

- Nick Mosher (Team Coordinator/VC Coordinator)
- Leon Kuhne (Integration Coordinator)
- Bryce Murphy (Requirements Coordinator/Configuration Coordinator)
- Fikayo Olutunji (Quality Assurance Coordinator) 

## Prerequisites

A typical development flow requires installing the following tools:

* [Rust](https://rustup.rs)
* [Docker](https://docs.docker.com/v17.09/engine/installation/#desktop)
* [Docker Compose](https://docs.docker.com/compose/install/)

## How to begin

Start by launching the development docker-compose image. This will start a
postgres database on port 5432 which the backend service can use.

```
docker-compose -f deployments/development.yml up
```

After postgres is running, navigate to the `backend/` directory and use
the `diesel_cli` tool to initialize the database and run the migrations.

```
cd backend/
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration run
```

At this point you can connect to postgres using a db client (such as IntelliJ
Ultimate), the development credentials for the database are as follows:

```
POSTGRES_DB=inventory
POSTGRES_USER=postgres
POSTGRES_PASSWORD=inventory-development
```

After running the diesel migrations, you should see the application's tables
in postgres. In order to tell the backend service how to connect to the
database, we use the `DATABASE_URL` environment variable, which can be set by
placing a definition in the `.env` file. An example `.env.dev` file is
provided with the development username, password, and database name filled out.
You can get started right away by simply copying `.env.dev` to `.env`.

```
# In backend/
cp .env.dev .env
```

Finally, launch the backend service using cargo:

```
cargo run
```

## License

MIT License

See LICENSE for details.
