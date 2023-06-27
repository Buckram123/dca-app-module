use crate::contract::{AppResult, DCAApp};
use crate::msg::{AppQueryMsg, ConfigResponse};
use crate::state::CONFIG;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

pub fn query_handler(deps: Deps, _env: Env, _app: &DCAApp, msg: AppQueryMsg) -> AppResult<Binary> {
    match msg {
        AppQueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
    .map_err(Into::into)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { config })
}
