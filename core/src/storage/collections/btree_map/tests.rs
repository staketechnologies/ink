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

use crate::{
    storage::{
        alloc::{
            AllocateUsing,
            BumpAlloc,
            Initialize,
        },
        Key,
        BTreeMap,
    },
    test_utils::run_test,
};
use crate::storage::btree_map::impls::Entry;

fn empty_map() -> BTreeMap<i32, i32> {
    unsafe {
        let mut alloc = BumpAlloc::from_raw_parts(Key([0x0; 32]));
        BTreeMap::allocate_using(&mut alloc).initialize_into(())
    }
}

fn filled_map() -> BTreeMap<i32, i32> {
    let mut map = empty_map();
    map.insert(5, 50);
    map.insert(42, 420);
    map.insert(1337, 13370);
    map.insert(77, 770);
    assert_eq!(map.len(), 4);
    map
}

#[test]
fn new_unchecked() {
    run_test(|| {
        let map = empty_map();
        // Initial invariant.
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        //assert_eq!(stash.iter().next(), None);
    })
}

#[test]
fn put_empty() {
    run_test(|| {
        let mut map = empty_map();
        // Before and after first put.
        assert_eq!(map.insert(42, 420), None);
        assert_eq!(map.insert(42, 520), Some(420));
        assert_eq!(map.get(&42), Some(&520));
    })
}

#[test]
fn first_put_filled() {
    run_test(|| {
        let mut map = filled_map();
        assert_eq!(map.get(&5), Some(&50));

        assert_eq!(map.get(&42), Some(&420));
        assert_eq!(map.get(&1337), Some(&13370));
        assert_eq!(map.get(&77), Some(&770));
        assert_eq!(map.get(&4), None);
        assert_eq!(map.len(), 4);

        assert_eq!(map.insert(4, 40), None);

        assert_eq!(map.get(&4), Some(&40));
        assert_eq!(map.len(), 5);
    })
}

#[test]
fn put_filled2() {
    run_test(|| {
        let mut map = empty_map();
        let mut len  = map.len();
        for i in 1..200 {
            assert_eq!(map.insert(i, i * 10), None);
            len += 1;
            assert_eq!(map.len(), len);
        }

        for i in 1..200 {
            assert_eq!(map.get(&i), Some(&(i * 10)));
        }
    })
}

#[test]
fn entry_api() {
    run_test(|| {
        let mut map = filled_map();
        assert_eq!(map.entry(5).key(), &5);
        assert_eq!(map.entry(-1).key(), &-1);

        assert_eq!(map.entry(997).or_insert(9970), &9970);
    });
}

#[test]
fn entry_api2() {
    run_test(|| {
        let mut map = unsafe {
            let mut alloc = BumpAlloc::from_raw_parts(Key([0x0; 32]));
            BTreeMap::allocate_using(&mut alloc).initialize_into(())
        };
        map.entry(String::from("poneyland")).or_insert(12);
        let p = String::from("poneyland");
        //assert_eq!(map[p], 12);
        if let Entry::Occupied(mut o) = map.entry(p) {
            *o.get_mut() += 10;
            assert_eq!(*o.get(), 22);

            // We can use the same Entry multiple times.
            *o.get_mut() += 2;
        }
        //assert_eq!(map[p], 24);
        let p = String::from("poneyland");
        assert_eq!(map.get(&p).expect("must be there"), &24);
    });
}


// TODO test flushing with *mut return values from entry api

/*

#[test]
fn take_empty() {
    run_test(|| {
        let mut stash = empty_stash();
        assert_eq!(stash.take(0), None);
        assert_eq!(stash.take(1000), None);
    })
}

#[test]
fn take_filled() {
    run_test(|| {
        let mut stash = filled_stash();
        // Take and check len
        assert_eq!(stash.len(), 4);
        assert_eq!(stash.take(0), Some(5));
        assert_eq!(stash.len(), 3);
        assert_eq!(stash.take(1), Some(42));
        assert_eq!(stash.len(), 2);
        assert_eq!(stash.take(2), Some(1337));
        assert_eq!(stash.len(), 1);
        assert_eq!(stash.take(3), Some(77));
        assert_eq!(stash.len(), 0);
        assert_eq!(stash.take(4), None);
        assert_eq!(stash.len(), 0);
    })
}

#[test]
fn put_take() {
    run_test(|| {
        let mut stash = filled_stash();
        // Take and put "randomly"
        //
        // Layout of the stash in memory:
        //
        //          |----------------------------------|
        // Index    |    0 |    1 |    2 |    3 |    4 |
        //          |------|------|------|------|------|
        // Occupied |    5 |   42 | 1337 |   77 |      |
        // Vacant   |      |      |      |      |      |
        //          |----------------------------------|
        // next_vacant = 4
        assert_eq!(stash.take(2), Some(1337));
        //          |----------------------------------|
        // Index    |    0 |    1 |    2 |    3 |    4 |
        //          |------|------|------|------|------|
        // Occupied |    5 |   42 |      |   77 |      |
        // Vacant   |      |      |    4 |      |      |
        //          |----------------------------------|
        // next_vacant = 2
        assert_eq!(stash.take(0), Some(5));
        //          |----------------------------------|
        // Index    |    0 |    1 |    2 |    3 |    4 |
        //          |------|------|------|------|------|
        // Occupied |      |   42 |      |   77 |      |
        // Vacant   |    2 |      |    4 |      |      |
        //          |----------------------------------|
        // next_vacant = 0
        assert_eq!(stash.put(123), 0);
        //          |----------------------------------|
        // Index    |    0 |    1 |    2 |    3 |    4 |
        //          |------|------|------|------|------|
        // Occupied |  123 |   42 |      |   77 |      |
        // Vacant   |      |      |    4 |      |      |
        //          |----------------------------------|
        // next_vacant = 2
        assert_eq!(stash.put(555), 2);
        //          |----------------------------------|
        // Index    |    0 |    1 |    2 |    3 |    4 |
        //          |------|------|------|------|------|
        // Occupied |  123 |   42 |  555 |   77 |      |
        // Vacant   |      |      |      |      |      |
        //          |----------------------------------|
        // next_vacant = 4
        assert_eq!(stash.put(999), 4);
        //          |------------------------------------------
        // Index    |    0 |    1 |    2 |    3 |    4 |    5 |
        //          |------|------|------|------|------|------|
        // Occupied |  123 |   42 |  555 |   77 |  999 |      |
        // Vacant   |      |      |      |      |      |      |
        //          |------------------------------------------
        // next_vacant = 5
        assert_eq!(stash.take(3), Some(77));
        //          |------------------------------------------
        // Index    |    0 |    1 |    2 |    3 |    4 |    5 |
        //          |------|------|------|------|------|------|
        // Occupied |  123 |   42 |  555 |      |  999 |      |
        // Vacant   |      |      |      |    5 |      |      |
        //          |------------------------------------------
        // next_vacant = 3
        assert_eq!(stash.take(0), Some(123));
        //          |------------------------------------------
        // Index    |    0 |    1 |    2 |    3 |    4 |    5 |
        //          |------|------|------|------|------|------|
        // Occupied |      |   42 |  555 |      |  999 |      |
        // Vacant   |    3 |      |      |    5 |      |      |
        //          |------------------------------------------
        // next_vacant = 0
        assert_eq!(stash.put(911), 0);
        //          |------------------------------------------
        // Index    |    0 |    1 |    2 |    3 |    4 |    5 |
        //          |------|------|------|------|------|------|
        // Occupied |  911 |   42 |  555 |      |  999 |      |
        // Vacant   |      |      |      |    5 |      |      |
        //          |------------------------------------------
        // next_vacant = 3
        assert_eq!(stash.take(3), None);
        assert_eq!(stash.take(1), Some(42));
        //          |------------------------------------------
        // Index    |    0 |    1 |    2 |    3 |    4 |    5 |
        //          |------|------|------|------|------|------|
        // Occupied |  911 |      |  555 |      |  999 |      |
        // Vacant   |      |    3 |      |    5 |      |      |
        //          |------------------------------------------
        // next_vacant = 1
    })
}

#[test]
fn iter() {
    run_test(|| {
        let stash = filled_stash();
        let mut iter = stash.iter();
        assert_eq!(iter.next(), Some((0, &5)));
        assert_eq!(iter.next(), Some((1, &42)));
        assert_eq!(iter.next(), Some((2, &1337)));
        assert_eq!(iter.next(), Some((3, &77)));
        assert_eq!(iter.next(), None);
    })
}

fn holey_stash() -> Stash<i32> {
    let mut stash = filled_stash();
    stash.put(123);
    stash.take(1);
    stash.take(3);
    stash
}

#[test]
fn iter_holey() {
    run_test(|| {
        let stash = holey_stash();
        let mut iter = stash.iter();
        assert_eq!(iter.next(), Some((0, &5)));
        assert_eq!(iter.next(), Some((2, &1337)));
        assert_eq!(iter.next(), Some((4, &123)));
        assert_eq!(iter.next(), None);
    })
}

#[test]
fn iter_back() {
    run_test(|| {
        let stash = filled_stash();
        let mut iter = stash.iter();
        assert_eq!(iter.next_back(), Some((3, &77)));
        assert_eq!(iter.next_back(), Some((2, &1337)));
        assert_eq!(iter.next_back(), Some((1, &42)));
        assert_eq!(iter.next_back(), Some((0, &5)));
        assert_eq!(iter.next_back(), None);
    })
}

#[test]
fn iter_back_holey() {
    run_test(|| {
        let stash = holey_stash();
        let mut iter = stash.iter();
        assert_eq!(iter.next_back(), Some((4, &123)));
        assert_eq!(iter.next_back(), Some((2, &1337)));
        assert_eq!(iter.next_back(), Some((0, &5)));
        assert_eq!(iter.next_back(), None);
    })
}

#[test]
fn iter_size_hint() {
    run_test(|| {
        let stash = filled_stash();
        let mut iter = stash.iter();
        assert_eq!(iter.size_hint(), (4, Some(4)));
        iter.next();
        assert_eq!(iter.size_hint(), (3, Some(3)));
    })
}
*/
