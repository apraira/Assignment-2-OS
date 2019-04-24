**Nama: Salsa Rahmadati**

**Exercism username: salsarahmadati**

# Assignment 2 Operating System
Assalamualaikum warahmatullahi wabarakatuh, alhamdulillah saya dapat menyelesaikan tugas ini tepat pada waktunya.
Tugas ini disusun untuk mata kuliah Sistem Operasi yang dibimbing oleh pak Eka. 

Saya menyelesaikan 5 problem medium:
- Hamming
- Isogram
- Clock
- Perfect Number
- Triangles

Dalam esai ini saya akan menjelaskan bagaimana cara menyelesaikan masalah level medium, "Perfect Number", yang terdapat di https://exercism.io .




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

Hal yang pertama saya lakukan adalah membuat jawaban dari pertanyaan "bagaimana cara menjumlahkan faktor-faktor dari sebuah bilangan tanpa bilangan itu sendiri", saya membuat fungsi baru bernama sum yang mempunyai tipe u64, sama seperti tipe yang ada di fungsi classify. Fungsi ini berfungsi untuk menjumlahkan faktor-faktor tidak termasuk bilangan itu sendiri. Fungsi ini hanya akan saya panggil ketika x > 0.

```rust
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
```
**Penjelasan:**
- Fungsi ini dipanggil dengan sum(x) dimana x adalah bilangan bertipe u64.
- Setelah dipanggil, maka akan ada variabel count yang disini saya beri nama sum, variabel ini saya inisiasi terlebih dahulu dengan 0.
- Lalu kita menjalankan indeks (i) dari 1 sampai bilangan x.
- Jika bilangan x habis dibagi i, maka variabel sum yang tadinya 0 akan ditambah dengan nilai i.
- Proses ini akan selesai ketika i = x.
- Dan akan mengembalikan nilai dari variabel sum itu sendiri.

Setelah itu saya menambahkan _if & else_ untuk mengidentifikasi apakah bilangan asli itu adalah _Perfect_, _Deficient_, atau, _Abundant_. Fungsi classify ini mempunyai _input_ dengan tipe data u64 dan _output_ dengan tipe data Option. 

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
**Penjelasan:**
- Pada fungsi ini, saya memanggil fungsi sum(x) yang sudah saya buat sebelumnya.
- Dengan memanggil fungsi sum(x) dan mengkombinasikannya dengan _if & else_ maka saya dapat dengan mudah mengklasifikasikan mana yang Sempurna, Kelebihan, maupun Kekurangan sesuai dengan ketentuan yang berlaku.

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



 
