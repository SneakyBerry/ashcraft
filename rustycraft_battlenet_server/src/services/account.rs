use crate::Server;
use rustycraft_protocol::bgs::protocol::account::v1::{
    AccountFieldTags, AccountService, AccountState, GameAccountFieldTags, GameAccountState,
    GameLevelInfo, GameStatus, GetAccountStateRequest, GetAccountStateResponse,
    GetGameAccountStateRequest, GetGameAccountStateResponse, PrivacyInfo,
};
use rustycraft_protocol::errors::WowRpcResponse;

#[async_trait::async_trait]
impl AccountService for Server {
    async fn get_account_state(
        &mut self,
        request: GetAccountStateRequest,
    ) -> Result<GetAccountStateResponse, WowRpcResponse> {
        Ok(GetAccountStateResponse {
            state: Some(AccountState {
                account_level_info: None,
                privacy_info: if request
                    .options
                    .as_ref()
                    .unwrap()
                    .field_privacy_info
                    .unwrap_or(false)
                {
                    Some(PrivacyInfo {
                        is_using_rid: Some(false),
                        is_visible_for_view_friends: Some(false),
                        is_hidden_from_friend_finder: Some(true),
                        game_info_privacy: None,
                        only_allow_friend_whispers: None,
                    })
                } else {
                    None
                },
                parental_control_info: None,
                game_level_info: vec![],
                game_status: vec![],
                game_accounts: vec![],
                security_status: None,
                government_curfew: None,
            }),
            tags: if request
                .options
                .as_ref()
                .unwrap()
                .field_privacy_info
                .unwrap_or(false)
            {
                Some(AccountFieldTags {
                    account_level_info_tag: None,
                    privacy_info_tag: Some(0xD7CA834D),
                    parental_control_info_tag: None,
                    game_level_info_tags: vec![],
                    game_status_tags: vec![],
                    game_account_tags: vec![],
                    security_status_tag: None,
                })
            } else {
                None
            },
        })
    }

    async fn get_game_account_state(
        &mut self,
        request: GetGameAccountStateRequest,
    ) -> Result<GetGameAccountStateResponse, WowRpcResponse> {
        let mut tags = GameAccountFieldTags {
            game_level_info_tag: None,
            game_time_info_tag: None,
            game_status_tag: None,
            raf_info_tag: None,
        };
        let mut state = GameAccountState {
            game_level_info: None,
            game_time_info: None,
            game_status: None,
            raf_info: None,
        };
        if let Some(opts) = request.options {
            tags.game_level_info_tag = Some(0x5C46D483);
            if opts.field_game_level_info.unwrap_or(false) {
                state.game_level_info = Some(GameLevelInfo {
                    is_trial: None,
                    is_lifetime: None,
                    is_restricted: None,
                    is_beta: None,
                    name: Some("Handlefa1".to_owned()),
                    program: Some(5730135),
                    licenses: vec![],
                    realm_permissions: None,
                    last_logout_time_ms: None,
                });
            };
            if opts.field_game_status.unwrap_or(false) {
                state.game_status = Some(GameStatus {
                    is_suspended: Some(false),
                    is_banned: Some(false),
                    suspension_expires: None,
                    program: Some(5730135),
                    is_locked: Some(false),
                    is_bam_unlockable: Some(false),
                });
            }
        }
        let response = GetGameAccountStateResponse {
            state: Some(state),
            tags: Some(tags),
        };
        Ok(response)
    }
}
