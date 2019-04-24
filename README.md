Nama: Salsa Rahmadati
, Exercism username: salsarahmadati

# Assignment 2 Operating System
Assalamualaikum warahmatullahi wabarakatuh, alhamdulillah saya dapat menyelesaikan tugas ini tepat pada waktunya.
Tugas ini disusun untuk mata kuliah Sistem Operasi yang dibimbing oleh pak Eka. 

Saya menyelesaikan 5 problem medium:
- Hamming
- Isogram
- Clock
- Perfect Number
- Triangles

Dalam essay ini saya akan menjelaskan bagaimana cara menyelesaikan masalah level medium, "Perfect Number", yang terdapat di https://exercism.io .




## Perfect Number Solution
Problem ini meminta kita untuk menentukan apakah sebuah bilangan itu merupakan sempurna (perfect), berlimpah(abundant), atau kurang(deficient) berdasarkan skema klasifikasi Nicomachus (60M -120M) untuk bilangan asli.

Pengidentifikasian suatu bilangan asli itu perfect, abundant, atau deficient didapatkan dari penjumlahan faktor-faktor dari suatu bilangan tidak termasuk bilangan itu sendiri atau yang disebut dengan alikuot.


## Ketentuan
1. Sempurna (Perfect): Jumlah alikuot = bilangan
  > 6 adalah bilangan sempurna karena (1 + 2 + 3) = 6
  
2. Berlimpah (Abundant): Jumlah alikuot > bilangan
  > 12 adalah abundant karena (1 + 2 + 3 + 4 + 6) = 16
  
3. Kekurangan (Deficient): jumlah alikuot < bilangan
  > 8 adalah bilangan yang deficient karena (1 + 2 + 4) = 7

## Solusi saya

Pertama saya membuat fungsi baru untuk menjumlahkan alikuot:

```rust
pub fn sum(x:u64) -> u64{
    let mut sum = 0; //membuat variabel baru
    if x > 0 {
        for i in 1..x{
            if x%i == 0 { //jika x habis dibagi i
            sum = sum + i; //maka sum yang tadinya 0 ditambah dengan value dari i
            }else{
            sum = sum; //kalau gak habis sum tetap
            }
            
        }
    }
    sum
    
}
```

Setelah itu di fungsi classify kita membuat if&else:

```rust

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
```

## Full Code

```rust


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
    if x > 0 {
        for i in 1..x{
            if x%i == 0 { //jika x habis dibagi i
            sum = sum + i; //maka sum yang tadinya 0 ditambah dengan value dari i
            }else{
            sum = sum; //kalau gak habis sum tetap
            }
            
        }
    }
    sum
    
}
```



 
