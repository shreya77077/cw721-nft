use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};

// use crate::msg::{CustomResponse, ExecuteMsg, QueryMsg};
 use cw721_base::{CustomResponse, ExecuteMsg, QueryMsg};
// use cw721_base::{CustomResponse, ExecuteMsg, QueryMsg};
// use cw721_base::CustomResponse;
// // use cw721_base::QueryMsg;
// use cw721_base::ExecuteMsg;
// use cw721_base::QueryMsg;
//use cw721_base::{ExecuteMsg as Cw721ExecuteMsg, QueryMsg as Cw721QueryMsg};




/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this. Rename it to your contract name.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

 pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
    // pub fn call<T, E>(msg: T) -> StdResult<CosmosMsg> where T: Into<ExecuteMsg<E>>, E: StdError {
            // Function implementation
        // }
        
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    /// Get Custom
    pub fn custom_query<Q, T, CQ>(&self, querier: &Q, val: String) -> StdResult<CustomResponse>
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::CustomMsg { val };
        // let msg = Cw721QueryMsg::CustomMsg {val  };

        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: CustomResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }
}
