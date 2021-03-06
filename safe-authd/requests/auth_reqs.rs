// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use crate::shared::{lock_auth_reqs_list, SharedAuthReqsHandle};
use safe_api::PendingAuthReqs;
use serde_json::{json, Value};

pub fn process_req(params: Value, auth_reqs_handle: SharedAuthReqsHandle) -> Result<Value, String> {
    if Value::Null != params {
        Err(format!(
            "Unexpected param for 'auth-reqs' method: {:?}",
            params
        ))
    } else {
        println!("Obtaining list of pending authorisation requests...");
        let pending_auth_reqs: PendingAuthReqs =
            lock_auth_reqs_list(auth_reqs_handle, |auth_reqs_list| {
                Ok(auth_reqs_list
                    .iter()
                    .map(|(_req_id, pending_req)| pending_req.auth_req.clone())
                    .collect())
            })?;

        println!(
            "List of pending authorisation requests sent: {:?}",
            pending_auth_reqs
        );
        Ok(json!(pending_auth_reqs))
    }
}
