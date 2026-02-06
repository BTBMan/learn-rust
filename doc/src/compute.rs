///!
/// Compute something
///

/**
 * `test` fn returns sum of two specific numbers
 *
 * # Examples
 * ```
 * let answer = doc::compute::test(1, 2);
 * assert_eq!(answer, 3);
 * ```
 */
pub fn test(left: i32, right: i32) -> i32 {
    left + right
}

/**
 * Panics
 *
 * ```should_panic
 * doc::compute::can_panic();
 * ```
 */
pub fn can_panic() {
    panic!("xxxxxxxxx")
}

/**
 * Hide
 *
 * ```
 * # doc::compute::hide_test_in_doc();
 * ```
 */
pub fn hide_test_in_doc() {}

/// return [`i32`] type
///
/// [`std::io::Write`]
///
/// jump to [`can_panic`](fn@can_panic)
#[doc(alias = "jum")]
pub fn jump() -> i32 {
    5
}
