// Copyright 2018-2019 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::storage::BTreeMap;
use core::{
    cmp::{Ord, Ordering},
    borrow::Borrow,
};
use scale::Codec;
use crate::storage::btree_map::impls::{
    KVHandle,
    NodeHandle,
    HandleType,
    Node,
};

pub enum SearchResult {
    Found(KVHandle),
    GoDown(KVHandle)
}

pub fn search_tree<K, V, Q>(
    btree: &BTreeMap<K, V>,
    current_root: u32,
    key: &Q
) -> SearchResult
where
    Q: Ord,
    K: Ord + Borrow<Q> + core::fmt::Debug + Codec,
    V: core::fmt::Debug + Codec,
{
    if btree.len() == 0 {
        return SearchResult::GoDown(
            KVHandle{
                node: current_root,
                idx: 0,
            }
        );
    }

    let mut cur = current_root;
    loop {
        let node = btree.get_node(&NodeHandle(cur));
        match search_node(&node, cur, key) {
            SearchResult::Found(handle) => return SearchResult::Found(handle),
            SearchResult::GoDown(handle) => {
                match get_handle_type(btree, handle) {
                    HandleType::Leaf(leaf) => return SearchResult::GoDown(leaf),
                    HandleType::Internal(internal) => {
                        cur = btree.descend(internal).0;
                        continue;
                    }
                }
            }
        }
    }
}

pub fn search_node<K, V, Q>(
    node: &Node<K, V>,
    node_index: u32,
    key: &Q,
) -> SearchResult
where
      Q: Ord,
      K: Borrow<Q>,
{
    match search_linear(node, key) {
        (idx, true) => SearchResult::Found(
            //Handle::new_kv(index, idx as u32)
            KVHandle {
                node: node_index,
                idx,
            }
        ),
        (idx, false) => SearchResult::GoDown(
            //Handle::new_edge(index, idx as u32)
            KVHandle {
                node: node_index,
                idx,
            }
        )
    }
}

pub fn search_linear<K, V, Q>(
    node: &Node<K, V>,
    key: &Q
) -> (u32, bool)
where
      Q: Ord,
      K: Borrow<Q>,
{
    let iter = node.keys.iter().enumerate();
    for (i, k) in iter {
        match k {
            None => return (i as u32, false),
            Some(node_key) => {
                match key.cmp(node_key.borrow()) {
                    Ordering::Greater => {},
                    Ordering::Equal => return (i as u32, true),
                    Ordering::Less => return (i as u32, false)
                }
            }
        }
    }
    // ToDo maybe return KVHandle instead of u32
    (node.len, false)
}

fn get_handle_type<K, V>(btree: &BTreeMap<K, V>, handle: KVHandle) -> HandleType
where
    K: Ord + core::fmt::Debug + Codec,
    V: core::fmt::Debug + Codec,
{
    let node = btree.get_node(&NodeHandle(handle.node));
    let children = node.edges.iter().filter(|e| e.is_some()).count();
    if children == 0 {
        HandleType::Leaf(handle)
    } else {
        HandleType::Internal(handle)
    }
}
