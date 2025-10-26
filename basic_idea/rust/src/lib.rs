// Safe: Uses checked arithmetic (panics in debug, needs explicit handling in release)
pub fn calculate_buffer_size_checked(width: u32, height: u32) -> Option<u32> {
    width.checked_mul(height)?.checked_mul(4)
}

// Unchecked: Wraps silently (like C unsigned behavior)
pub fn calculate_buffer_size_wrapping(width: u32, height: u32) -> u32 {
    width.wrapping_mul(height).wrapping_mul(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_case() {
        assert_eq!(calculate_buffer_size_checked(1024, 768), Some(3145728));
    }

    #[test]
    fn test_overflow_case() {
        // This should overflow: u32::MAX / 2 * 3 > u32::MAX
        assert_eq!(calculate_buffer_size_checked(u32::MAX / 2, 3), None);
    }
}