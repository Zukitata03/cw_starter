#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
use crate::state:{Config, CONFIG, Poll, POLLS};

const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or(info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;
    let config = Config {
        admin:validated_admin.clone(),

    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "instantiate").add_attribute("admin", validated_admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll {
            poll_id,
            question,
            options,
        } => execute_create_poll(deps, env, info, poll id, question, options),
        ExecuteMsg::Vote { poll_id, vote } => unimplemented!(),
    }
    
}

fn execute_create_poll(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    if options.len() > 10 {
        return Err(ContractError::TooManyOptions {});
    }
    let mut opts: Vec<(String, u64)> = vec![];
    for option in options {
        opts.push((option, 0));
    }

    let poll = Poll {
        creator: info.sender,
        question,
        options, opts
    };

    POLLS.save(deps.storage, &poll_id, &poll)?;
    Ook(Response:new()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::attr; // hellper to construct atributtes eg. atr("action", "instantiate")
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use crate::contract::instantiate;
    use crate::msg::InstantiateMsg; //instance method

    //fake address
    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]); //empty vec means no funds sent

        let msg = InstantiateMsg { admin: None }; // sender will be the admin
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap(); //unwrap to get the result

        assert_eq!(
            res.attributes,
            vec![
                attr("action", "instantiate"),
                attr("admin", ADDR1),
            ]
        );
    }

    fn test_instantiate_with_admin() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]); //empty vec means no funds sent
        
        let msg = InstantiateMsg { admin: Some(ADDR2.to_string()) }; // sender will be the admin
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap(); //unwrap to get the result

        assert_eq!(
            res.attributes,
            vec![
                attr("action", "instantiate"),
                attr("admin", ADDR2),
            ]
        );
    }

}
