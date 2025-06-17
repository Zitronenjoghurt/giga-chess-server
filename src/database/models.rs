pub mod invite_code;
pub mod user;

pub trait Model {
    type NewModel;
    type PrimaryKeyType;
}
