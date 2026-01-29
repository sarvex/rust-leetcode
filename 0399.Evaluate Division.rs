use std::collections::HashMap;

struct UnionFind {
    parent: HashMap<String, String>,
    weight: HashMap<String, f64>,
}

impl UnionFind {
    fn new(equations: &[Vec<String>]) -> Self {
        let mut parent = HashMap::new();
        let mut weight = HashMap::new();
        for eq in equations {
            for var in eq {
                parent.entry(var.clone()).or_insert_with(|| var.clone());
                weight.entry(var.clone()).or_insert(1.0);
            }
        }
        Self { parent, weight }
    }

    fn find(&mut self, x: &str) -> String {
        let p = self.parent[x].clone();
        if p == x {
            return p;
        }
        let root = self.find(&p);
        let pw = self.weight[&p];
        self.weight.entry(x.to_string()).and_modify(|w| *w *= pw);
        self.parent.insert(x.to_string(), root.clone());
        root
    }

    fn union(&mut self, a: &str, b: &str, value: f64) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        let (wa, wb) = (self.weight[a], self.weight[b]);
        self.parent.insert(ra.clone(), rb);
        self.weight.insert(ra, wb * value / wa);
    }

    fn query(&mut self, a: &str, b: &str) -> f64 {
        if !self.parent.contains_key(a) || !self.parent.contains_key(b) {
            return -1.0;
        }
        let ra = self.find(a);
        let rb = self.find(b);
        if ra != rb {
            -1.0
        } else {
            self.weight[a] / self.weight[b]
        }
    }
}

impl Solution {
    /// Evaluates division queries using a weighted Union-Find.
    ///
    /// # Intuition
    /// Each equation a/b = v establishes a ratio relationship. Union-Find with
    /// weights tracks the cumulative ratio from each variable to its root,
    /// enabling O(alpha(n)) query evaluation.
    ///
    /// # Approach
    /// 1. Build a weighted Union-Find from the equations.
    /// 2. For each query, find both roots; if they match, compute weight_a / weight_b.
    /// 3. Return -1.0 for disconnected or unknown variables.
    ///
    /// # Complexity
    /// - Time: O((E + Q) * alpha(N)) where E = equations, Q = queries
    /// - Space: O(N) for the Union-Find structure
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut uf = UnionFind::new(&equations);
        for (eq, &val) in equations.iter().zip(values.iter()) {
            uf.union(&eq[0], &eq[1], val);
        }
        queries.iter().map(|q| uf.query(&q[0], &q[1])).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_eq(pairs: &[(&str, &str)]) -> Vec<Vec<String>> {
        pairs
            .iter()
            .map(|(a, b)| vec![a.to_string(), b.to_string()])
            .collect()
    }

    #[test]
    fn test_basic() {
        let equations = make_eq(&[("a", "b"), ("b", "c")]);
        let values = vec![2.0, 3.0];
        let queries = make_eq(&[("a", "c"), ("b", "a"), ("a", "e"), ("a", "a"), ("x", "x")]);
        let result = Solution::calc_equation(equations, values, queries);
        let expected = vec![6.0, 0.5, -1.0, 1.0, -1.0];
        for (r, e) in result.iter().zip(expected.iter()) {
            assert!((r - e).abs() < 1e-5, "expected {e}, got {r}");
        }
    }

    #[test]
    fn test_single_equation() {
        let equations = make_eq(&[("a", "b")]);
        let values = vec![2.0];
        let queries = make_eq(&[("a", "b"), ("b", "a")]);
        let result = Solution::calc_equation(equations, values, queries);
        assert!((result[0] - 2.0).abs() < 1e-5);
        assert!((result[1] - 0.5).abs() < 1e-5);
    }

    #[test]
    fn test_chain() {
        let equations = make_eq(&[("a", "b"), ("b", "c"), ("c", "d")]);
        let values = vec![2.0, 3.0, 4.0];
        let queries = make_eq(&[("a", "d")]);
        let result = Solution::calc_equation(equations, values, queries);
        assert!((result[0] - 24.0).abs() < 1e-5);
    }

    #[test]
    fn test_disconnected() {
        let equations = make_eq(&[("a", "b"), ("c", "d")]);
        let values = vec![2.0, 3.0];
        let queries = make_eq(&[("a", "c")]);
        let result = Solution::calc_equation(equations, values, queries);
        assert!((result[0] - (-1.0)).abs() < 1e-5);
    }

    #[test]
    fn test_same_variable() {
        let equations = make_eq(&[("a", "b")]);
        let values = vec![2.0];
        let queries = make_eq(&[("a", "a"), ("b", "b")]);
        let result = Solution::calc_equation(equations, values, queries);
        assert!((result[0] - 1.0).abs() < 1e-5);
        assert!((result[1] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_unknown_variable() {
        let equations = make_eq(&[("a", "b")]);
        let values = vec![2.0];
        let queries = make_eq(&[("x", "y")]);
        let result = Solution::calc_equation(equations, values, queries);
        assert!((result[0] - (-1.0)).abs() < 1e-5);
    }
}
