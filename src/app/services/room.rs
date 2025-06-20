use crate::api::models::general::pagination::Pagination;
use crate::api::models::query::pagination::PaginationQuery;
use crate::api::models::response::room_info::PublicRoomInfo;
use crate::api::models::response::room_list::PublicRoomList;
use crate::app::config::Config;
use crate::app::error::{AppError, AppResult};
use crate::app::services::Service;
use crate::database::stores::room::RoomStore;
use crate::database::stores::user::UserStore;
use crate::database::stores::{Store, Stores};
use futures::future::try_join_all;
use std::sync::Arc;

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

                Ok::<PublicRoomInfo, AppError>(PublicRoomInfo::from_room_and_players(
                    room,
                    white.as_ref(),
                    black.as_ref(),
                ))
            })
            .collect::<Vec<_>>();

        let room_infos = try_join_all(room_info_futures).await?;
        let pagination = Pagination::from_query(&pagination_query, results, total);

        Ok(PublicRoomList {
            rooms: room_infos,
            pagination,
        })
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
