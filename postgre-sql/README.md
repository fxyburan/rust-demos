```sql
CREATE TABLE "user" (
    uid SERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    registration_time TIMESTAMP NOT NULL,
    login_time TIMESTAMP NOT NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL
);
```