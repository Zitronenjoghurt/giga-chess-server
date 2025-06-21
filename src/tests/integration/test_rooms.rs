use crate::api::models::body::room_creation::RoomCreationBody;
use crate::api::models::response::room_list::PublicRoomList;
use crate::tests::build_test_server_with_user;

const USERNAME: &str = "test_user";
const PASSWORD: &str = "testtest";

#[tokio::test]
async fn test_rooms() {
    let (server, _, jwt) = build_test_server_with_user(USERNAME, PASSWORD).await;

    let room_creation = RoomCreationBody {
        name: Some("Get Rekt".to_string()),
        public: true,
        time_micros: None,
        increment_micros: None,
    };
    let creation_response = server
        .post("/room")
        .add_header("Authorization", format!("Bearer {}", jwt))
        .json(&room_creation)
        .await;
    assert_eq!(
        creation_response.status_code(),
        201,
        "Room creation failed with status {}: {}",
        creation_response.status_code(),
        creation_response.text()
    );

    let room_creation2 = RoomCreationBody {
        name: None,
        public: false,
        time_micros: None,
        increment_micros: None,
    };
    let creation_response2 = server
        .post("/room")
        .add_header("Authorization", format!("Bearer {}", jwt))
        .json(&room_creation2)
        .await;
    assert_eq!(
        creation_response2.status_code(),
        201,
        "2nd room creation failed with status {}: {}",
        creation_response.status_code(),
        creation_response.text()
    );

    let public_rooms_response = server
        .get("/room")
        .add_header("Authorization", format!("Bearer {}", jwt))
        .await;
    assert_eq!(
        public_rooms_response.status_code(),
        200,
        "Public rooms fetching failed with status {}: {}",
        public_rooms_response.status_code(),
        public_rooms_response.text()
    );

    let public_rooms: PublicRoomList = public_rooms_response.json();
    assert_eq!(public_rooms.rooms.len(), 1);
    assert_eq!(public_rooms.rooms[0].name, Some("Get Rekt".to_string()));
    assert_eq!(public_rooms.pagination.total, 1);
    assert_eq!(public_rooms.pagination.results, 1);
    assert_eq!(public_rooms.pagination.page, 1);
    assert_eq!(public_rooms.pagination.limit, 10);
}
