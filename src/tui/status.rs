
pub enum Status {
    Completed,
    Pending,
    OnHold,
    Create,
    Invalid
}


impl Status {
    pub fn get_enum(c: u8) -> Self {
        match c {
            0 => { Status::Completed },
            1 => { Status::Pending   },
            2 => { Status::OnHold    },
            3 => { Status::Create    },
            _ => { Status::Invalid   }
        }
    }

    pub fn get_code(&self) -> u8 {
        match self {
            Status::Completed => { 0 },
            Status::Pending   => { 1 },
            Status::OnHold    => { 2 },
            Status::Create    => { 3 },
            Status::Invalid   => { 9 }
        }
    }

    pub fn get_string(&self) -> &str {
        match self {
            Status::Completed => { "Completed" },
            Status::Pending   => { "Pending" },
            Status::OnHold    => { "OnHold" },
            Status::Create    => { "Create" },
            Status::Invalid   => { "Invalid" }
        }
    }

    pub fn set(&mut self, s: Self) {
        *self = s
    }
}
