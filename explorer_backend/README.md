# Stegos API server and fetcher
This is API server and fetcher for stegos blockchain explorer. Check it out at [explorer.stegos.com](http://explorer.stegos.com).

## Stegos API server
Stegos API server provide a GraphQL API for blockchain explorers. 
Our api server contain documentation in playground form at [`/graphiql` page](http://ex01.stegos.com/graphiql). 

## Project setup
This project is written on `Rust`, and uses `PostgresQL` + `Diesel ORM` for its database.
In order to hack with it, you need to download and install Rust. Check out https://rustup.rs/ for more information.

### Managing database
In order to start Postgres, we suggest to use [docker image](https://hub.docker.com/_/postgres).

Using command like this should be enough:
```
docker run --volume "$PWD/data/postgres:/var/lib/postgresql/data" -p "5432:5432"  postgres:12.1-alpine
```

Next you will need diesel cli installed:

```
cargo install diesel_cli --no-default-features --features postgres
```

Don't use manipulate database directly, for changing database relations use Diesel_cli.

For initializing database use command:

```
diesel setup
```

[Checkout diesel manual](http://diesel.rs/guides/getting-started/), for more information about diesel operations.

# Fetcher service

Fetcher is a worker service that loads history from stegosd using Websocket API into postgres database.
Each fetcher can download only from single node. So in order to support testnet and mainnet 2 fetchers should be runned.


# Starting localy

For starting all stack locally you can use docker-compose.


Go to the [deploy folder](../deploy) and run 

```
docker-compose up
```

This should start postgres, fetcher, and api services.