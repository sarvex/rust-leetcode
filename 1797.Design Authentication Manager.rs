use std::collections::HashMap;

/// Token-based authentication manager with time-to-live expiration.
struct AuthenticationManager {
    time_to_live: i32,
    tokens: HashMap<String, i32>,
}

impl AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        Self {
            time_to_live,
            tokens: HashMap::new(),
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.tokens
            .insert(token_id, current_time + self.time_to_live);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        if self
            .tokens
            .get(&token_id)
            .is_some_and(|expiry| *expiry > current_time)
        {
            self.tokens
                .insert(token_id, current_time + self.time_to_live);
        }
    }

    fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
        self.tokens
            .values()
            .filter(|expiry| **expiry > current_time)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_flow() {
        let mut mgr = AuthenticationManager::new(5);
        mgr.generate("aaa".to_string(), 1);
        assert_eq!(mgr.count_unexpired_tokens(6), 0);
        mgr.generate("bbb".to_string(), 7);
        mgr.renew("aaa".to_string(), 8);
        assert_eq!(mgr.count_unexpired_tokens(10), 1);
    }

    #[test]
    fn test_renew_expired_token() {
        let mut mgr = AuthenticationManager::new(3);
        mgr.generate("token1".to_string(), 1);
        mgr.renew("token1".to_string(), 5);
        assert_eq!(mgr.count_unexpired_tokens(5), 0);
    }
}
