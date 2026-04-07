const DIRS: [&str; 4] = ["East", "North", "West", "South"];

struct Robot {
    perimeter: i32,
    pos: i32,
    x: i32,
    y: i32,
    dir: usize,
    seg1: i32,
    seg2: i32,
    seg3: i32,
    width: i32,
    height: i32,
}

impl Robot {
    /// Perimeter-ring robot with eager coordinate caching.
    ///
    /// # Intuition
    /// Flatten the grid perimeter into a cyclic 1-D ring. Each `step` advances
    /// the ring index and eagerly resolves the new (x, y, direction), so that
    /// `get_pos` and `get_dir` are pure field reads with no branching.
    ///
    /// # Approach
    /// Ring of length `2*(w + h - 2)` split into four segments:
    /// - `[0, w)` East, `[w, w+h-1)` North, `[w+h-1, 2w+h-2)` West,
    ///   `[2w+h-2, perimeter)` South.
    /// Position 0 after moving means the robot completed a full lap (South).
    ///
    /// # Complexity
    /// - Time: O(1) per operation
    /// - Space: O(1)
    #[inline]
    fn new(width: i32, height: i32) -> Self {
        Self {
            perimeter: 2 * (width + height - 2),
            pos: 0,
            x: 0,
            y: 0,
            dir: 0,
            seg1: width,
            seg2: width + height - 1,
            seg3: 2 * width + height - 2,
            width,
            height,
        }
    }

    #[inline]
    fn step(&mut self, num: i32) {
        self.pos = (self.pos + num) % self.perimeter;
        let p = self.pos;
        if p == 0 {
            self.x = 0;
            self.y = 0;
            self.dir = 3;
        } else if p < self.seg1 {
            self.x = p;
            self.y = 0;
            self.dir = 0;
        } else if p < self.seg2 {
            self.x = self.width - 1;
            self.y = p - self.seg1 + 1;
            self.dir = 1;
        } else if p < self.seg3 {
            self.x = self.seg3 - 1 - p;
            self.y = self.height - 1;
            self.dir = 2;
        } else {
            self.x = 0;
            self.y = self.perimeter - p;
            self.dir = 3;
        }
    }

    #[inline]
    fn get_pos(&self) -> Vec<i32> {
        vec![self.x, self.y]
    }

    #[inline]
    fn get_dir(&self) -> String {
        String::from(DIRS[self.dir])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut robot = Robot::new(6, 3);
        robot.step(2);
        robot.step(2);
        assert_eq!(robot.get_pos(), vec![4, 0]);
        assert_eq!(robot.get_dir(), "East");
        robot.step(2);
        robot.step(1);
        robot.step(4);
        assert_eq!(robot.get_pos(), vec![1, 2]);
        assert_eq!(robot.get_dir(), "West");
    }

    #[test]
    fn full_perimeter_returns_to_origin_facing_south() {
        let mut robot = Robot::new(3, 3);
        robot.step(8);
        assert_eq!(robot.get_pos(), vec![0, 0]);
        assert_eq!(robot.get_dir(), "South");
    }

    #[test]
    fn no_steps_faces_east() {
        let robot = Robot::new(2, 2);
        assert_eq!(robot.get_pos(), vec![0, 0]);
        assert_eq!(robot.get_dir(), "East");
    }

    #[test]
    fn single_step() {
        let mut robot = Robot::new(4, 3);
        robot.step(1);
        assert_eq!(robot.get_pos(), vec![1, 0]);
        assert_eq!(robot.get_dir(), "East");
    }

    #[test]
    fn corners() {
        let mut robot = Robot::new(3, 3);
        robot.step(2);
        assert_eq!(robot.get_pos(), vec![2, 0]);
        assert_eq!(robot.get_dir(), "East");

        robot.step(2);
        assert_eq!(robot.get_pos(), vec![2, 2]);
        assert_eq!(robot.get_dir(), "North");

        robot.step(2);
        assert_eq!(robot.get_pos(), vec![0, 2]);
        assert_eq!(robot.get_dir(), "West");

        robot.step(2);
        assert_eq!(robot.get_pos(), vec![0, 0]);
        assert_eq!(robot.get_dir(), "South");
    }

    #[test]
    fn large_steps() {
        let mut robot = Robot::new(100, 100);
        robot.step(100_000);
        assert_eq!(robot.get_pos(), vec![89, 99]);
        assert_eq!(robot.get_dir(), "West");
    }

    #[test]
    fn minimum_grid() {
        let mut robot = Robot::new(2, 2);
        robot.step(1);
        assert_eq!(robot.get_pos(), vec![1, 0]);
        assert_eq!(robot.get_dir(), "East");
        robot.step(1);
        assert_eq!(robot.get_pos(), vec![1, 1]);
        assert_eq!(robot.get_dir(), "North");
        robot.step(1);
        assert_eq!(robot.get_pos(), vec![0, 1]);
        assert_eq!(robot.get_dir(), "West");
        robot.step(1);
        assert_eq!(robot.get_pos(), vec![0, 0]);
        assert_eq!(robot.get_dir(), "South");
    }
}
