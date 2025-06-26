use crate::app::config::Config;
use crate::app::error::{AppError, AppResult};
use crate::app::services::Service;
use crate::database::models::room::Room;
use crate::database::models::user::User;
use crate::database::stores::room::RoomStore;
use crate::database::stores::user::UserStore;
use crate::database::stores::{Store, Stores};
use futures::future::try_join_all;
use giga_chess_api_types::general::pagination::Pagination;
use giga_chess_api_types::query::pagination::PaginationQuery;
use giga_chess_api_types::response::room_info::PublicRoomInfo;
use giga_chess_api_types::response::room_list::PublicRoomList;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct RoomService {
    room_store: Arc<RoomStore>,
    user_store: Arc<UserStore>,
}

impl RoomService {
    pub async fn public_room_list(
        &self,
        pagination_query: PaginationQuery,
    ) -> AppResult<PublicRoomList> {
        let (rooms, total) = self
            .room_store
            .list_public(pagination_query.page(), pagination_query.limit())
            .await?;
        let results = rooms.len() as i64;

        let room_info_futures = rooms
            .iter()
            .map(async |room| {
                let white = match room.player_white {
                    Some(uuid) => self.user_store.find(uuid).await?,
                    None => None,
                };

                let black = match room.player_black {
                    Some(uuid) => self.user_store.find(uuid).await?,
                    None => None,
                };

                Ok::<PublicRoomInfo, AppError>(room.get_public_info(white.as_ref(), black.as_ref()))
            })
            .collect::<Vec<_>>();

        let room_infos = try_join_all(room_info_futures).await?;
        let pagination = Pagination::from_query(&pagination_query, results, total);

        Ok(PublicRoomList {
            rooms: room_infos,
            pagination,
        })
    }

    pub async fn join(&self, uuid: Uuid, user: &User) -> AppResult<Room> {
        let Some(mut room) = self.room_store.find(uuid).await? else {
            return Err(AppError::not_found("Room"));
        };

        if room.created_by == user.id {
            return Err(AppError::bad_request("Cannot join your own room"));
        }

        if !room.public {
            return Err(AppError::not_found("Room"));
        }

        let success = room.join(user);
        if !success {
            return Err(AppError::bad_request("Room is full"));
        }

        self.room_store.save(room).await
    }
}

impl Service for RoomService {
    fn initialize(_config: &Arc<Config>, stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            room_store: stores.room.clone(),
            user_store: stores.user.clone(),
        })
    }
}
