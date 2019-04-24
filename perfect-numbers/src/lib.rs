

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num > 0 { //jika num lebih besar dari 0 maka lanjut 
        if sum(num) == num { //cek lagi apakah summ dari faktor itu sama 
            return Some(Classification::Perfect) }  //jika iya maka Perfect
        else if sum(num) > num { //Jika lebih dari jumlah sum
        Some(Classification::Abundant) //maka Abundant
        } else { //Jika kurang dari
        Some(Classification::Deficient) //Maka deficient
        }
    } else {
        None //jika bilangan kurang dari 0 returnnya None
    }
}

pub fn sum(x:u64) -> u64{
    let mut sum = 0; //membuat variabel baru
        for i in 1..x{
            if x%i == 0 { //jika x habis dibagi i
            sum = sum + i; //maka sum yang tadinya 0 ditambah dengan value dari i
            }else{
            sum = sum; //kalau gak habis sum tetap
            }
    
        }
    sum
}

// Problem link: https://exercism.io/tracks/rust/exercises/perfect-numbers/solutions/ffd4330ee24d49bca7d21d6cc310edf1
