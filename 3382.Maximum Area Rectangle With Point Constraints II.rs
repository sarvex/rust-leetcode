#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 1, 3, 3], vec![1, 3, 1, 3]),
            4
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 1, 3, 3, 2], vec![1, 3, 1, 3, 2]),
            -1
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 1, 3, 3, 1, 3], vec![1, 3, 1, 3, 2, 2]),
            2
        );
    }

    #[test]
    fn test_insufficient_points() {
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 2, 3], vec![1, 2, 3]),
            -1
        );
    }

    #[test]
    fn test_no_valid_rectangle() {
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 2, 3, 4], vec![1, 2, 3, 4]),
            -1
        );
    }

    #[test]
    fn test_multiple_rectangles() {
        assert_eq!(
            Solution::max_rectangle_area(vec![0, 0, 1, 1, 2, 2], vec![0, 1, 0, 1, 0, 1]),
            1
        );
    }

    #[test]
    fn test_large_rectangle() {
        assert_eq!(
            Solution::max_rectangle_area(vec![0, 0, 10, 10], vec![0, 20, 0, 20]),
            200
        );
    }

    #[test]
    fn test_point_on_horizontal_edge() {
        assert_eq!(
            Solution::max_rectangle_area(
                vec![
                    32, 100, 32, 100, 85, 12, 59, 84, 3, 68, 31, 75, 87, 83, 30, 22, 85, 71, 92, 69
                ],
                vec![0, 11, 11, 0, 42, 83, 80, 0, 18, 43, 18, 25, 12, 62, 94, 27, 76, 19, 14, 33]
            ),
            -1
        );
    }
}
