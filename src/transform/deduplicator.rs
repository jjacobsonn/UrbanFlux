// Deduplication logic
use std::collections::HashSet;
use tracing::debug;

#[derive(Debug)]
pub struct Deduplicator {
    seen_keys: HashSet<i64>,
}

impl Deduplicator {
    pub fn new() -> Self {
        Self {
            seen_keys: HashSet::new(),
        }
    }

    /// Check if a key has been seen before
    pub fn is_duplicate(&mut self, key: i64) -> bool {
        !self.seen_keys.insert(key)
    }

    /// Get the count of unique keys seen
    pub fn unique_count(&self) -> usize {
        self.seen_keys.len()
    }

    /// Clear the deduplicator state
    pub fn clear(&mut self) {
        self.seen_keys.clear();
        debug!("Deduplicator cleared");
    }
}

impl Default for Deduplicator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deduplication() {
        let mut dedup = Deduplicator::new();
        
        assert!(!dedup.is_duplicate(1));
        assert!(dedup.is_duplicate(1));
        assert!(!dedup.is_duplicate(2));
        assert_eq!(dedup.unique_count(), 2);
    }

    #[test]
    fn test_clear() {
        let mut dedup = Deduplicator::new();
        dedup.is_duplicate(1);
        dedup.is_duplicate(2);
        
        dedup.clear();
        assert_eq!(dedup.unique_count(), 0);
        assert!(!dedup.is_duplicate(1));
    }
}
