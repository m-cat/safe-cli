// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

#[cfg(not(any(feature = "fake-auth", feature = "scl-mock")))]
mod auth_and_connect;
pub mod auth_daemon;
pub mod config;
#[cfg(any(feature = "fake-auth", feature = "scl-mock"))]
mod fake_auth;

#[cfg(not(any(feature = "fake-auth", feature = "scl-mock")))]
pub mod safe_net {
    pub use super::auth_and_connect::*;
}
#[cfg(any(feature = "fake-auth", feature = "scl-mock"))]
pub mod safe_net {
    pub use super::fake_auth::*;
}
