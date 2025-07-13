Below is the equivalent Rust ORM (*Diesel*) model for your SQL schema.

**Key points:**
- In Diesel, the field types should map closely to SQL. 
- Use chrono for `Timestamp` with time zone.
- Nullable fields use Option.
- Snake_case recommended, but you can specify custom DB column names with `#[diesel(column_name = "...")]` if you want to keep casing.

### 1. Diesel Table Macro

First, in your `schema.rs`:

```rust
// schema.rs

diesel::table! {
    users (id) {
        id -> BigInt,
        password -> Varchar,
        last_login -> Nullable<Timestamptz>,
        is_superuser -> Bool,
        email -> Varchar,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        is_active -> Bool,
        is_staff -> Bool,
        mfa_enabled -> Bool,
        email_verifield -> Bool,
        date_joined -> Timestamptz,
    }
}
```

### 2. The Model Struct

```rust
// models.rs

use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub password: String,
    pub last_login: Option<DateTime<Utc>>,
    pub is_superuser: bool,
    pub email: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub is_active: bool,
    pub is_staff: bool,
    pub mfa_enabled: bool,
    pub email_verifield: bool,
    pub date_joined: DateTime<Utc>,
}
```

### 3. Notes

- Make sure you add `chrono = { version = "0.4", features = ["serde"] }` and `diesel` with `postgres` enabled to your `Cargo.toml`.
- `Timestamptz` = `chrono::DateTime<Utc>`
- The struct field names should match the table columns or use #[diesel(column_name = "...")] for custom names.

---

Let me know if you also want an `Insertable` struct, or if you have additional columns or relations to model!