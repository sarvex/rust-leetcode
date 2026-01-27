impl Solution {
    /// Greedy subtraction of largest Fibonacci numbers.
    ///
    /// # Intuition
    /// Using the largest Fibonacci number not exceeding the remainder at each
    /// step is optimal by Zeckendorf's theorem, guaranteeing the minimum count.
    ///
    /// # Approach
    /// 1. Precompute Fibonacci numbers up to 10^9 in descending order
    /// 2. Greedily subtract the largest Fibonacci ≤ k, incrementing count
    /// 3. Stop when k reaches 0
    ///
    /// # Complexity
    /// - Time: O(log k) — number of Fibonacci numbers up to k
    /// - Space: O(1) with a const array
    pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
        const FIB: [i32; 45] = [
            1836311903, 1134903170, 701408733, 433494437, 267914296, 165580141, 102334155,
            63245986, 39088169, 24157817, 14930352, 9227465, 5702887, 3524578, 2178309, 1346269,
            832040, 514229, 317811, 196418, 121393, 75025, 46368, 28657, 17711, 10946, 6765, 4181,
            2584, 1597, 987, 610, 377, 233, 144, 89, 55, 34, 21, 13, 8, 5, 3, 2, 1,
        ];

        let mut count = 0;
        for &f in &FIB {
            if k >= f {
                k -= f;
                count += 1;
                if k == 0 {
                    break;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_seven() {
        assert_eq!(Solution::find_min_fibonacci_numbers(7), 2);
    }

    #[test]
    fn example_ten() {
        assert_eq!(Solution::find_min_fibonacci_numbers(10), 2);
    }

    #[test]
    fn fibonacci_number() {
        assert_eq!(Solution::find_min_fibonacci_numbers(13), 1);
    }

    #[test]
    fn one() {
        assert_eq!(Solution::find_min_fibonacci_numbers(1), 1);
    }
}
