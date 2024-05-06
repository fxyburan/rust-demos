### Install PostgreSQL
```bash
sudo apt-get install postgre-sql
```

### Connect DB with the user postgres
```bash
psql -U postgres
```

### Connect DB with the new user
```bash
psql -U clique clique_mpc_db
```

```sql
CREATE TABLE "user" (
    uid SERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    registration_time TIMESTAMP NOT NULL,
    login_time TIMESTAMP NOT NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL
);
```

