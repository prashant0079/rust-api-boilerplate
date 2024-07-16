// Context Data Model

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64,
}

impl Ctx {
    // Constructor.
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }

    // Property Accessors
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
