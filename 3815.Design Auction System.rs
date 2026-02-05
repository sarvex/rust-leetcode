//! Design Auction System
//!
//! BinaryHeap per item with lazy deletion: add/update only push, remove only clears
//! from the canonical map; get_highest_bidder pops stale heap entries until top matches.

use std::collections::{BinaryHeap, HashMap};

#[inline(always)]
fn key(user_id: i32, item_id: i32) -> u32 {
    (user_id as u32) << 16 | (item_id as u32)
}

struct AuctionSystem {
    /// packed(user_id, item_id) -> current bid amount (canonical)
    bid_amount_map: HashMap<u32, i32>,
    /// item_id -> max-heap (bid_amount, user_id); tie-break higher user_id
    by_item: HashMap<i32, BinaryHeap<(i32, i32)>>,
}

impl AuctionSystem {
    fn new() -> Self {
        Self {
            bid_amount_map: HashMap::new(),
            by_item: HashMap::new(),
        }
    }

    fn add_bid(&mut self, user_id: i32, item_id: i32, bid_amount: i32) {
        let k = key(user_id, item_id);
        self.bid_amount_map.entry(k).and_modify(|v| *v = bid_amount).or_insert(bid_amount);
        self.by_item
            .entry(item_id)
            .or_default()
            .push((bid_amount, user_id));
    }

    fn update_bid(&mut self, user_id: i32, item_id: i32, new_amount: i32) {
        self.add_bid(user_id, item_id, new_amount);
    }

    fn remove_bid(&mut self, user_id: i32, item_id: i32) {
        self.bid_amount_map.remove(&key(user_id, item_id));
    }

    fn get_highest_bidder(&mut self, item_id: i32) -> i32 {
        let Some(hp) = self.by_item.get_mut(&item_id) else {
            return -1;
        };
        while let Some(&(bid_amount, user_id)) = hp.peek() {
            let k = key(user_id, item_id);
            if self.bid_amount_map.get(&k).copied() == Some(bid_amount) {
                return user_id;
            }
            hp.pop();
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut sys = AuctionSystem::new();
        sys.add_bid(1, 7, 5);
        sys.add_bid(2, 7, 6);
        assert_eq!(sys.get_highest_bidder(7), 2);
        sys.update_bid(1, 7, 8);
        assert_eq!(sys.get_highest_bidder(7), 1);
        sys.remove_bid(2, 7);
        assert_eq!(sys.get_highest_bidder(7), 1);
        assert_eq!(sys.get_highest_bidder(3), -1);
    }

    #[test]
    fn test_replace_bid_same_user() {
        let mut sys = AuctionSystem::new();
        sys.add_bid(1, 1, 10);
        sys.add_bid(1, 1, 20);
        assert_eq!(sys.get_highest_bidder(1), 1);
    }

    #[test]
    fn test_tie_break_highest_user_id() {
        let mut sys = AuctionSystem::new();
        sys.add_bid(1, 1, 5);
        sys.add_bid(2, 1, 5);
        assert_eq!(sys.get_highest_bidder(1), 2);
    }

    #[test]
    fn test_no_bids() {
        let mut sys = AuctionSystem::new();
        assert_eq!(sys.get_highest_bidder(1), -1);
    }
}
