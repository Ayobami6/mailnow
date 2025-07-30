Certainly! For Rust, with [`diesel`](https://diesel.rs/) as the ORM, you would model the schema with a combination of a Diesel migration and the corresponding Rust struct. Below is a complete translation:

---

### 1. Diesel migration (`up.sql`)

You would typically express the table in a migration like:

```sql
CREATE TABLE team_members (
    id BIGSERIAL PRIMARY KEY,
    role VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    company_id BIGINT NOT NULL REFERENCES companies(id),
    user_id BIGINT NOT NULL REFERENCES users(id)
);
```

---

### 2. Diesel table macro (`schema.rs`)

You can let Diesel infer this, but for completeness:

```rust
table! {
    team_members (id) {
        id -> Int8,
        role -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        company_id -> Int8,
        user_id -> Int8,
    }
}
```
If you're using `diesel print-schema` this will be auto-generated.

---

### 3. Rust model (`models.rs`)

```rust
use chrono::NaiveDateTime; // or chrono::DateTime<Utc> for TIMESTAMPTZ with chrono features
use diesel::{Queryable, Insertable};
use crate::schema::team_members;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "team_members"]
pub struct TeamMember {
    pub id: i64,
    pub role: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub company_id: i64,
    pub user_id: i64,
}
```
**Notes:**
- `TIMESTAMPTZ` in Diesel with Postgres maps to `chrono::DateTime<Utc>`.
- You must enable Diesel's `"chrono"` feature (`diesel = { version = "...", features = ["chrono"] }` in `Cargo.toml`).

---

### 4. (Optional) New Insertable struct

Often you'll want an "Insertable" struct without the PK (since PKs can auto-increment):

```rust
#[derive(Insertable)]
#[table_name = "team_members"]
pub struct NewTeamMember<'a> {
    pub role: &'a str,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub company_id: i64,
    pub user_id: i64,
}
```

---

Let me know if you also want the relations (joins with users/companies) modeled!