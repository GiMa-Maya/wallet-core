// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

use crate::signing_input::MayachainSigningInput;
use tw_coin_entry::coin_context::CoinContext;
use tw_cosmos_sdk::context::StandardCosmosContext;
use tw_cosmos_sdk::modules::signer::tw_signer::TWSigner;
use tw_proto::Cosmos::Proto;

pub struct MatachainSigner;

impl MayachainSigner {
    pub fn sign(
        coin: &dyn CoinContext,
        mut input: Proto::SigningInput<'_>,
    ) -> Proto::SigningOutput<'static> {
        MayachainSigningInput::prepare_signing_input(&mut input);
        TWSigner::<StandardCosmosContext>::sign(coin, input)
    }
}
