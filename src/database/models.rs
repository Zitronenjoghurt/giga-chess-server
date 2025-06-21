pub mod invite_code;
pub mod room;
pub mod session;
pub mod user;

pub trait Model {
    type NewModel;
    type PrimaryKeyType;
}
