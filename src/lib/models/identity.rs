use uuid::Uuid;

/// # Identity
/// Represents the core of any identity in the system including users
#[allow(dead_code)]
#[derive(Clone)]
pub struct Identity {
    uid: Uuid,
    email: String,
    username: String,
}

impl Default for Identity {
    fn default() -> Self {
        Identity {
            uid: Uuid::new_v4(),
            email: String::new(),
            username: String::new(),
        }
    }
}
