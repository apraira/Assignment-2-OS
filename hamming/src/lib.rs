/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let mut x = 0;
    if s1.len() != s2.len(){
        None
    } else {
        for i in 0..s1.len(){
            if s1.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap() {
                x = x + 1
                }
            else {
                x = x
            }
        }
        Some(x)
    }
    
}
