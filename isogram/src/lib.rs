use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut ans = candidate.to_string();
    ans.retain(|c| c != '-' && c != ' ');
    ans = ans.to_lowercase();
    let mut looked: HashSet<char> = HashSet::new();

    for i in ans.chars(){
        if looked.contains(&i){
            return false;
        }

        looked.insert(i);

    }
   true
}

//Problem link: https://exercism.io/tracks/rust/exercises/hamming/solutions/bd9a08fbdcec469495de90e82c715439
