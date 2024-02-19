# Database Migrations

## Before running any sql command

```shell
export DATABASE_URL=postgres://postgres:password@localhost:5437/newsletter
```

## Create subscriptions table

```shell
sqlx migrate add create_subscriptions_table
```