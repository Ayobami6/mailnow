Sure! Let's transform your SQL schema to a [Diesel](https://diesel.rs/) ORM model in Rust.

First, let’s define the schema in Diesel’s DSL (`schema.rs`) style, and then the struct (model) that maps to it.

## Diesel `schema.rs`

```rust
table! {
    api_keys (id) {
        id -> BigInt,
        name -> Varchar,
        api_key -> Varchar,
        created_at -> Timestamptz,
        last_used -> Nullable<Timestamptz>,
        expires_at -> Nullable<Timestamptz>,
        is_active -> Bool,
        company_id -> BigInt,
        permission -> Nullable<Varchar>,
    }
}

table! {
    companies (id) {
        id -> BigInt,
        // ... other fields (define as needed)
    }
}

joinable!(api_keys -> companies (company_id));

allow_tables_to_appear_in_same_query!(api_keys, companies);
```

## Model Struct

```rust
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Varchar, Timestamptz, Bool};
use chrono::{DateTime, Utc};

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Company))]
#[table_name = "api_keys"]
pub struct ApiKey {
    pub id: i64,
    pub name: String,
    pub api_key: String,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub company_id: i64,
    pub permission: Option<String>,
}

#[derive(Queryable, Identifiable)]
#[table_name = "companies"]
pub struct Company {
    pub id: i64,
    // ... other fields
}
```

### Notes

- Use `chrono::DateTime<Utc>` for `timestamp with time zone` columns in Diesel/Rust.
- Nullable columns are represented as `Option<T>`
- The field names and struct field types directly correspond to table columns.
- If you want to insert, use `Insertable` as well.

Let me know if you’d like a full `Insertable` struct, or if the companies table should be more fully declared!