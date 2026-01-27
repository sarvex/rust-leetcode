/// Array-based parking system with slot capacity tracking.
///
/// # Intuition
/// Track available slots per car type (big=1, medium=2, small=3) in a
/// fixed-size array. Each `add_car` call decrements the corresponding
/// counter if space remains.
///
/// # Complexity
/// - new: O(1)
/// - add_car: O(1)
/// - Space: O(1)
struct ParkingSystem {
    slots: [i32; 4],
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            slots: [0, big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let slot = &mut self.slots[car_type as usize];
        if *slot == 0 {
            return false;
        }
        *slot -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parking_operations() {
        let mut ps = ParkingSystem::new(1, 1, 0);
        assert!(ps.add_car(1));
        assert!(ps.add_car(2));
        assert!(!ps.add_car(3));
        assert!(!ps.add_car(1));
    }
}
