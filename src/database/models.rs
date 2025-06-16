pub mod user;

pub trait Model {
    type NewModel;
    type PrimaryKeyType;
}
