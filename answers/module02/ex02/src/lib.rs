pub enum PizzaStatus {
    Ordered,
    Cooking,
    Cooked,
    Delivering,
    Delivered,
}

impl PizzaStatus {
    pub fn from_delivery_time(ordered_days_ago: u32) -> Self {
        match ordered_days_ago {
            0..=1 => Self::Ordered,
            2..=6 => Self::Cooking,
            7..=9 => Self::Cooked,
            10..=16 => Self::Delivering,
            _ => Self::Delivered,
        }
    }

    pub fn get_delivery_time_in_days(&self) -> u32 {
        match self {
            Self::Delivered => 0,
            Self::Delivering => 7,
            Self::Cooked => 10,
            Self::Cooking => 15,
            Self::Ordered => 17,
        }
    }
}
