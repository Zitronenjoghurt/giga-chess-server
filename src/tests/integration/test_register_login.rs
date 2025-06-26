use crate::database::models::invite_code::NewInviteCode;
use crate::database::stores::Store;
use crate::tests::build_test_server;
use giga_chess_api_types::body::login::LoginBody;
use giga_chess_api_types::body::register::RegisterBody;
use giga_chess_api_types::response::login::LoginResponse;
use giga_chess_api_types::response::message::MessageResponse;

const USERNAME: &str = "test_user";
const PASSWORD: &str = "silly goober 123";

#[tokio::test]
async fn test_register_login() {
    let (server, state) = build_test_server();

    // Create invite code
    let invite_code = state
        .stores
        .invite_code
        .create(NewInviteCode::new())
        .await
        .unwrap();

    // Register user
    let register_data = RegisterBody {
        invite_code: invite_code.id.to_string(),
        username: USERNAME.to_string(),
        password: PASSWORD.to_string(),
    };

    let registration_response = server.post("/register").json(&register_data).await;

    assert_eq!(
        registration_response.status_code(),
        201,
        "Registration failed with status {}: {}",
        registration_response.status_code(),
        registration_response.text()
    );

    let registration_json: MessageResponse = registration_response.json();
    assert_eq!(registration_json.message, "Successfully registered");

    // Check that user was created and invite code was used
    let user = state
        .stores
        .user
        .find_by_name(USERNAME)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user.invite_code_id, invite_code.id);

    let invite_code = state
        .stores
        .invite_code
        .find(invite_code.id)
        .await
        .unwrap()
        .unwrap();
    assert!(invite_code.used);

    // Login user
    let login_data = LoginBody {
        username: USERNAME.to_string(),
        password: PASSWORD.to_string(),
    };

    let login_response = server.post("/login").json(&login_data).await;

    assert_eq!(
        login_response.status_code(),
        200,
        "Login failed with status {}: {}",
        login_response.status_code(),
        login_response.text()
    );

    let login_json: LoginResponse = login_response.json();

    // Check that user was logged in
    let ping_response = server
        .get("/ping")
        .add_header("Authorization", format!("Bearer {}", login_json.token))
        .await;

    assert_eq!(
        ping_response.status_code(),
        200,
        "Ping failed with status {}: {}",
        ping_response.status_code(),
        ping_response.text()
    );

    let ping_json: MessageResponse = ping_response.json();
    assert_eq!(ping_json.message, format!("Hello, {}", USERNAME));
}
