

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0{
        return None;
    } 
    else if num > 0 {
    if sum(num) == num {
        return Some(Classification::Perfect)
    } else if sum(num) > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
    } else {
        None
    }
}

pub fn sum(x:u64) -> u64{
    let mut sum = 0;
    if x == 1 {
        return 0;
    } 
    else if x > 1 {
        for i in 1..x{
            if x%i == 0 {
            sum = sum + i;
            }else{
            sum = sum;
            }
            
        }
    }
    sum
    
}
