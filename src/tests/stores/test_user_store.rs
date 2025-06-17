use crate::database::models::invite_code::NewInviteCode;
use crate::database::models::user::NewUser;
use crate::database::stores::Store;
use crate::tests::build_test_app_state;

#[test]
fn test_crud() {
    let state = build_test_app_state();

    let invite_code = state
        .stores
        .invite_code
        .create(NewInviteCode::new())
        .unwrap();
    let new_user = NewUser::new("Test User", invite_code.id, "Cheesecake");

    let mut created_user = state.stores.user.create(new_user).unwrap();
    let found_user = state.stores.user.find(created_user.id).unwrap();
    assert_eq!(Some(created_user.clone()), found_user);

    created_user.name = "Blueberry Muffin".to_string();
    let saved_user = state.stores.user.save(created_user).unwrap();
    assert_eq!(saved_user.name, "Blueberry Muffin".to_string());

    let found_user = state.stores.user.find(saved_user.id).unwrap();
    assert_eq!(Some(saved_user.clone()), found_user);

    let deleted_user = state.stores.user.delete(saved_user.id).unwrap();
    assert_eq!(deleted_user.unwrap().name, "Blueberry Muffin".to_string());

    let not_found = state.stores.user.find(saved_user.id).unwrap();
    assert!(not_found.is_none());
}
