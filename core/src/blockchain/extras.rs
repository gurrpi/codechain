// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use ctypes::{H256, U256};

use heapsize::HeapSizeOf;

use super::super::header::BlockNumber;

/// Familial details concerning a block
#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct BlockDetails {
    /// Block number
    pub number: BlockNumber,
    /// Total score of the block and all its parents
    pub total_score: U256,
    /// Parent block hash
    pub parent: H256,
    /// List of children block hashes
    pub children: Vec<H256>,
}

impl HeapSizeOf for BlockDetails {
    fn heap_size_of_children(&self) -> usize {
        self.children.heap_size_of_children()
    }
}

/// Represents address of certain transaction within block
#[derive(Debug, PartialEq, Clone, RlpEncodable, RlpDecodable)]
pub struct TransactionAddress {
    /// Block hash
    pub block_hash: H256,
    /// Transaction index within the block
    pub index: usize
}

impl HeapSizeOf for TransactionAddress {
    fn heap_size_of_children(&self) -> usize { 0 }
}