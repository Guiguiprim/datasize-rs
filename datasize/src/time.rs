use super::{non_dynamic_const_heap_size, DataSize};

use time::{Date, Duration, Instant, OffsetDateTime, PrimitiveDateTime, Time};

non_dynamic_const_heap_size!(Date Duration Instant OffsetDateTime PrimitiveDateTime Time, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        assert!(!Date::IS_DYNAMIC);
        assert!(!Duration::IS_DYNAMIC);
        assert!(!Instant::IS_DYNAMIC);
        assert!(!OffsetDateTime::IS_DYNAMIC);
        assert!(!PrimitiveDateTime::IS_DYNAMIC);
        assert!(!Time::IS_DYNAMIC);
    }
}
