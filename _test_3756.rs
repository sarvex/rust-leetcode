struct Solution;

impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;

        let bytes = s.as_bytes();
        let n = bytes.len();

        let nz_count = bytes.iter().filter(|&&b| b != b'0').count();

        let mut prefix_sum = vec![0_i64; nz_count + 1];
        let mut prefix_num = vec![0_i64; nz_count + 1];
        let mut pows = vec![1_i64; nz_count + 1];

        let mut nz_left = vec![nz_count; n + 1];
        let mut nz_right = vec![usize::MAX; n];

        let mut ci = 0_usize;
        for i in 0..n {
            nz_left[i] = ci;
            if bytes[i] != b'0' {
                let d = i64::from(bytes[i] - b'0');
                prefix_num[ci + 1] = (prefix_num[ci] * 10 + d) % MOD;
                prefix_sum[ci + 1] = (prefix_sum[ci] + d) % MOD;
                pows[ci + 1] = pows[ci] * 10 % MOD;
                ci += 1;
            }
        }

        let mut last_nz = usize::MAX;
        for i in (0..n).rev() {
            if bytes[i] != b'0' {
                last_nz = nz_left[i];
            }
            nz_right[i] = last_nz;
        }

        let mut ans = Vec::with_capacity(queries.len());
        for query in &queries {
            let l = query[0] as usize;
            let r = query[1] as usize;

            let cl = nz_left[l];
            let cr = nz_right[r];

            if cl == nz_count || cr == usize::MAX || cl > cr {
                ans.push(0);
                continue;
            }

            let sum = (prefix_sum[cr + 1] - prefix_sum[cl] + MOD) % MOD;
            let len = cr + 1 - cl;
            let num = ((prefix_num[cr + 1] - prefix_num[cl] * pows[len]) % MOD + MOD) % MOD;

            ans.push((sum * num % MOD) as i32);
        }

        ans
    }
}

fn main() {
    // Example 1
    let s = "10203004".to_string();
    let queries = vec![vec![0_i32, 7], vec![1, 3], vec![4, 6]];
    let result = Solution::sum_and_multiply(s, queries);
    println!("Example 1: {:?} (expected [12340, 4, 9])", result);

    // Example 2
    let s = "1000".to_string();
    let queries = vec![vec![0_i32, 3], vec![1, 1]];
    let result = Solution::sum_and_multiply(s, queries);
    println!("Example 2: {:?} (expected [1, 0])", result);

    // Example 3
    let s = "9876543210".to_string();
    let queries = vec![vec![0_i32, 9]];
    let result = Solution::sum_and_multiply(s, queries);
    println!("Example 3: {:?} (expected [444444137])", result);

    // Detailed trace of "10203004"
    let s2 = "10203004";
    let bytes = s2.as_bytes();
    let nz_count = bytes.iter().filter(|&&b| b != b'0').count();
    println!("\n--- Trace for '{}' ---", s2);
    println!("nz_count: {}", nz_count);
    let mut nz_left = vec![nz_count; s2.len() + 1];
    let mut prefix_sum = vec![0_i64; nz_count + 1];
    let mut prefix_num = vec![0_i64; nz_count + 1];
    let mut pows = vec![1_i64; nz_count + 1];
    let mut ci = 0_usize;
    for i in 0..s2.len() {
        nz_left[i] = ci;
        if bytes[i] != b'0' {
            let d = i64::from(bytes[i] - b'0');
            prefix_num[ci + 1] = prefix_num[ci] * 10 + d;
            prefix_sum[ci + 1] = prefix_sum[ci] + d;
            pows[ci + 1] = pows[ci] * 10;
            ci += 1;
        }
    }
    println!("nz_left:    {:?}", nz_left);
    println!("prefix_sum: {:?}", prefix_sum);
    println!("prefix_num: {:?}", prefix_num);
    println!("pows:       {:?}", pows);
    let mut nz_right = vec![usize::MAX; s2.len()];
    let mut last_nz = usize::MAX;
    for i in (0..s2.len()).rev() {
        if bytes[i] != b'0' {
            last_nz = nz_left[i];
        }
        nz_right[i] = last_nz;
    }
    println!("nz_right:   {:?}", nz_right);
    println!("Query [0,7]: cl={} cr={}", nz_left[0], nz_right[7]);
    println!("Query [1,3]: cl={} cr={}", nz_left[1], nz_right[3]);
    println!("Query [4,6]: cl={} cr={}", nz_left[4], nz_right[6]);
}
