use super::{non_dynamic_const_heap_size, DataSize};

use uuid::Uuid;

non_dynamic_const_heap_size!(Uuid, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid() {
        assert!(!Uuid::IS_DYNAMIC);
    }
}
