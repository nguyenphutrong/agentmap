use tiktoken_rs::o200k_base;

#[derive(Clone)]
pub struct TokenCounter {
    bpe: tiktoken_rs::CoreBPE,
}

impl TokenCounter {
    pub fn new() -> Self {
        let bpe = o200k_base().expect("Failed to load o200k_base tokenizer");
        Self { bpe }
    }

    pub fn count(&self, text: &str) -> usize {
        self.bpe.encode_with_special_tokens(text).len()
    }

    pub fn count_bytes_estimate(text: &str) -> usize {
        text.len() / 4
    }
}

impl Default for TokenCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_counting() {
        let counter = TokenCounter::new();
        let text = "Hello, world! This is a test.";
        let tokens = counter.count(text);
        assert!(tokens > 0);
        assert!(tokens < text.len());
    }
}
