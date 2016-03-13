// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use util::{Address, H256, U256, Bytes};
use util::standard::*;
use ethcore::error::Error;
use ethcore::client::BlockChainClient;
use ethcore::block::ClosedBlock;
use ethcore::transaction::SignedTransaction;
use ethminer::{MinerService, MinerStatus};

pub struct TestMinerService;

impl MinerService for TestMinerService {

	/// Returns miner's status.
	fn status(&self) -> MinerStatus { unimplemented!(); }

	/// Imports transactions to transaction queue.
	fn import_transactions<T>(&self, _transactions: Vec<SignedTransaction>, _fetch_nonce: T) -> Result<(), Error> where T: Fn(&Address) -> U256 { unimplemented!(); }

	/// Returns hashes of transactions currently in pending
	fn pending_transactions_hashes(&self) -> Vec<H256> { unimplemented!(); }

	/// Removes all transactions from the queue and restart mining operation.
	fn clear_and_reset(&self, _chain: &BlockChainClient) { unimplemented!(); }

	/// Called when blocks are imported to chain, updates transactions queue.
	fn chain_new_blocks(&self, _chain: &BlockChainClient, _imported: &[H256], _invalid: &[H256], _enacted: &[H256], _retracted: &[H256]) { unimplemented!(); }

	/// New chain head event. Restart mining operation.
	fn prepare_sealing(&self, _chain: &BlockChainClient) { unimplemented!(); }

	/// Grab the `ClosedBlock` that we want to be sealed. Comes as a mutex that you have to lock.
	fn sealing_block(&self, _chain: &BlockChainClient) -> &Mutex<Option<ClosedBlock>> { unimplemented!(); }

	/// Submit `seal` as a valid solution for the header of `pow_hash`.
	/// Will check the seal, but not actually insert the block into the chain.
	fn submit_seal(&self, _chain: &BlockChainClient, _pow_hash: H256, _seal: Vec<Bytes>) -> Result<(), Error> { unimplemented!(); }
}