# Mail Subscriber

## Database Migration

### sqlx-cli install

```shell
cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres
```

```shell
sqlx --help
```

### Install Postgres Client

```shell
sudo pacman -S postgresql-libs
```

```shell
psql --help
```

### Create Subscriptions Table

```shell
export DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter
sqlx migrate add create_subscriptions_table                                                                                                                                                  ✔ 
Creating migrations/20241209054200_create_subscriptions_table.sql
```

Create the table definition in the file `migrations/{timestamp}_create_subscriptions_table.sql`:

```sql
CREATE TABLE subscriptions(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
);
```

### Run the migration

```shell
SKIP_DOCKER=true ./scripts/init_db.sh
```