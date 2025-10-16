# Development Workflow

## Database Schema Changes

### Process
1. **Django First**: Always start with Django models
2. **Run Migrations**: Apply changes to database
3. **Update Rust**: Sync Rust models and schema

### Steps

#### 1. Update Django Models
```bash
cd admin/
# Edit models in users/models.py or core/models.py
```

#### 2. Create and Run Migrations
```bash
cd admin/
python manage.py makemigrations
python manage.py migrate
```

#### 3. Update Rust Schema
```bash
cd api/
# Update src/schema.rs to match database changes
# Update src/models/users.rs structs
```

#### 4. Test Changes
```bash
cd api/
cargo check
cargo run
```

## Current API Credits Implementation

### Django Model (users/models.py)
```python
class Company(models.Model):
    # ... existing fields ...
    pricing_tier = models.CharField(max_length=50, choices=PRICING_TIERS, default='free')
    api_credits = models.BigIntegerField(default=20000)
    credits_reset_date = models.DateTimeField(default=timezone.now)
```

### Rust Model (src/models/users.rs)
```rust
pub struct Company {
    // ... existing fields ...
    pub pricing_tier: String,
    pub api_credits: i64,
    pub credits_reset_date: DateTime<Utc>,
}
```

### Pricing Tiers
- **Free**: 20,000 credits/month
- **Developer**: 100,000 credits/month  
- **Enterprise**: Unlimited credits

### Usage
- Credits auto-reset monthly for Free/Developer
- Enterprise has unlimited (no deduction)
- Dashboard shows remaining credits and reset date