fn squares(nums: &[i32]) -> Vec<i32> {
    nums.iter().map(|n| n * n).collect()
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maps_values() {
        assert_eq!(squares(&[1, 2, 3]), [1, 4, 9]);
    }
}
