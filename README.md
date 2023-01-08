# rust-api-slqlite

It's a learning repo for creating CRUD functionality with Rust and Sqlite.

## Routes

- GET `/`: heath check endpoint
- GET `/users`: return list stored users
- POST `/users`: create a new user
- GET `/users/{id}`: get user with the specified {id}
- PATCH `/users/{id}`: update user with the specified {id}
- DELETE `/users/{id}`: delete user with the specified {id}

## Starting the project

Run the following command:

```bash
cargo run
```

## Improvement points

- Handle errors properly & printing them
- return a meaningful error responses
