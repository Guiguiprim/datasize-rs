use super::{non_dynamic_const_heap_size, DataSize};

use prost_types::{Duration, Timestamp};

non_dynamic_const_heap_size!(Duration Timestamp, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prost_types() {
        assert!(!Duration::IS_DYNAMIC);
        assert!(!Timestamp::IS_DYNAMIC);
    }
}
