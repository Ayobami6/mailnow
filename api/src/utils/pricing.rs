use chrono::{DateTime, Datelike, Timelike, Utc};

#[derive(Debug, Clone)]
pub enum PricingTier {
    Free,
    Developer,
    Enterprise,
}

impl PricingTier {
    pub fn from_str(tier: &str) -> Self {
        match tier.to_lowercase().as_str() {
            "developer" => PricingTier::Developer,
            "enterprise" => PricingTier::Enterprise,
            _ => PricingTier::Free,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PricingTier::Free => "free".to_string(),
            PricingTier::Developer => "developer".to_string(),
            PricingTier::Enterprise => "enterprise".to_string(),
        }
    }

    pub fn monthly_credits(&self) -> i64 {
        match self {
            PricingTier::Free => 1_000,
            PricingTier::Developer => 10_000,
            PricingTier::Enterprise => -1, // Unlimited
        }
    }
}

pub fn get_next_reset_date() -> DateTime<Utc> {
    let now = Utc::now();
    let next_month = if now.month() == 12 {
        now.with_year(now.year() + 1)
            .unwrap()
            .with_month(1)
            .unwrap()
    } else {
        now.with_month(now.month() + 1).unwrap()
    };
    next_month
        .with_day(1)
        .unwrap()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
}

pub fn should_reset_credits(last_reset: DateTime<Utc>) -> bool {
    let now = Utc::now();
    now.year() > last_reset.year()
        || (now.year() == last_reset.year() && now.month() > last_reset.month())
}
