// Copyright 2019-2022 Manta Network.
// This file is part of manta-rs.
//
// manta-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// manta-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with manta-rs.  If not, see <http://www.gnu.org/licenses/>.

//! Signer HTTP Client Implementation

use crate::{
    config::{utxo::Address, Config, Parameters},
    signer::{
        client::network::{Message, Network},
        AssetMetadata, Checkpoint, GetRequest, IdentityRequest, IdentityResponse,
        InitialSyncRequest, SignError, SignRequest, SignResponse, SignWithTransactionDataResult,
        SyncError, SyncRequest, SyncResponse, TransactionDataRequest, TransactionDataResponse,
    },
};
use alloc::boxed::Box;
use manta_accounting::wallet::{self, signer};
use manta_util::{
    future::LocalBoxFutureResult,
    http::reqwest::{self, IntoUrl, KnownUrlClient},
    serde::{de::DeserializeOwned, Serialize},
};

#[doc(inline)]
pub use reqwest::Error;

/// Wallet Associated to [`Client`]
pub type Wallet<L> = wallet::Wallet<Config, L, Client>;

/// HTTP Signer Client
pub struct Client {
    /// Base Client
    base: KnownUrlClient,

    /// Network Selector
    network: Option<Network>,
}

impl Client {
    /// Builds a new HTTP [`Client`] that connects to `server_url`.
    #[inline]
    pub fn new<U>(server_url: U) -> Result<Self, Error>
    where
        U: IntoUrl,
    {
        Ok(Self {
            base: KnownUrlClient::new(server_url)?,
            network: None,
        })
    }

    /// Sets the network that will be used to wrap HTTP requests.
    #[inline]
    pub fn set_network(&mut self, network: Option<Network>) {
        self.network = network
    }

    /// Wraps the current outgoing `request` with a `network` if it is not `None`.
    #[inline]
    pub fn wrap_request<T>(&self, request: T) -> Message<T> {
        Message {
            network: self
                .network
                .expect("Unable to wrap request, missing network."),
            message: request,
        }
    }

    /// Sends a POST of type `command` with query string `request`.
    #[inline]
    pub async fn post_request<T, R>(&self, command: &str, request: T) -> reqwest::Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.base.post(command, &self.wrap_request(request)).await
    }
}

impl signer::Connection<Config> for Client {
    type AssetMetadata = AssetMetadata;
    type Checkpoint = Checkpoint;
    type Error = Error;

    #[inline]
    fn sync(
        &mut self,
        request: SyncRequest,
    ) -> LocalBoxFutureResult<Result<SyncResponse, SyncError>, Self::Error> {
        Box::pin(self.post_request("sync", request))
    }

    #[inline]
    fn sbt_sync(
        &mut self,
        request: SyncRequest,
    ) -> LocalBoxFutureResult<Result<SyncResponse, SyncError>, Self::Error> {
        Box::pin(self.post_request("sbt_sync", request))
    }

    #[inline]
    fn initial_sync(
        &mut self,
        request: InitialSyncRequest,
    ) -> LocalBoxFutureResult<Result<SyncResponse, SyncError>, Self::Error> {
        Box::pin(self.post_request("initial_sync", request))
    }

    #[inline]
    fn sign(
        &mut self,
        request: SignRequest,
    ) -> LocalBoxFutureResult<Result<SignResponse, SignError>, Self::Error> {
        Box::pin(self.post_request("sign", request))
    }

    #[inline]
    fn address(&mut self) -> LocalBoxFutureResult<Option<Address>, Self::Error> {
        Box::pin(self.post_request("address", GetRequest::Get))
    }

    #[inline]
    fn transaction_data(
        &mut self,
        request: TransactionDataRequest,
    ) -> LocalBoxFutureResult<TransactionDataResponse, Self::Error> {
        Box::pin(self.post_request("transaction_data", request))
    }

    #[inline]
    fn identity_proof(
        &mut self,
        request: IdentityRequest,
    ) -> LocalBoxFutureResult<IdentityResponse, Self::Error> {
        Box::pin(self.post_request("identity", request))
    }

    #[inline]
    fn sign_with_transaction_data(
        &mut self,
        request: SignRequest,
    ) -> LocalBoxFutureResult<SignWithTransactionDataResult, Self::Error> {
        Box::pin(self.post_request("sign_with_transaction_data", request))
    }

    #[inline]
    fn transfer_parameters(&mut self) -> LocalBoxFutureResult<Parameters, Self::Error> {
        Box::pin(self.post_request("transfer_parameters", GetRequest::Get))
    }
}
