fn main() {
  println!("hello world")
}


// tambahkan #[test] kalau ingin membuat unit test
#[test]
fn hello_test() {
  println!("Hello coyy")
}

// Variable
#[test]
fn test_variable() {
  let name = "Fizonenda"; // Variable bersifat immutable atau tidak bisa diubah
  // name = "anjay"; // Saat mencoba mengubah maka akan error "cannot mutate immutable variable"

  println!("Hello, {}", name);
}

// Variable mutable
#[test]
fn test_variable_mutable() {
  let mut name = "Fizonenda"; // Variable bisa di ubah dengan menambahkan "mut" setelah "let"
  println!("Hello, {}", name);
  
  name = "Nardi"; // Saat mencoba mengubah maka sudah tidak error
  println!("Hello, {}", name);
}

// Static typing
#[test]
fn static_typing() {
  let mut name = "Fizonenda"; // Variable bisa di ubah dengan menambahkan "mut" setelah "let"
  println!("Hello, {}", name);
  
  name = "Dea"; // Untuk mencegah warning mutable saja
  // name = 10; // Saat mencoba mengubah variable yang sebelumnya str ke int/uint maka akan error
  println!("Hello, {}", name);
}

// Shadowing
#[test]
fn shadowing() {
  let name = "Fizonenda"; // Variable ini akan tertutupi oleh varable "name" yang lain
  println!("Hello, {}", name);
  
  
  let name = 10; 
  /* 
  variable "let name" baru akan menutupi variable yang lama, sehingga
  saat kita memanggil "name" akan muncul isi variable yang baru
  */
  
  println!("Hello, {}", name);
}

/*
Tipe data
Rust membagi tipe data menjadi dua subsets; SCALAR & COMPOUND
Rust juga bisa mendeteksi tipe data (implisit) tetapi juga bisa jika mau
bisa juga memberi tipe data untuk variable (explisit)

SCALAR TYPE: 
- integer
- float
- boolean
- char

COMPOUND TYPE:
- tuple : [Kumpulan data yang memiliki tipe data yang berbeda]
- array : [Kumpulan data yang memiliki tipe data yang sama]
 */

 // Tipe data scalar
#[test]
fn number_and_float() {
  let num: i8 = 20; // Explicit
  println!("{}", num);

  let fl: f32 = 200.90; // Explicit
  println!("{}", fl);
}


#[test]
fn number_conversion() {
  let a: i8 = 20; // Explicit
  println!("{}", a);

  let b: i16 = a as i16; // Explicit
  println!("{}", b);

  // mengkonversi ke tipe data yang lebih besar dapat dilakukan 
  let c: i32 = b as i32;
  println!("{}", c);

  // Tapi hati hati kalau mengkonversi ke nilai yang lebih kecil
  let d: i64 = 100000000000000;
  let e: i8 = d as i8;
  // Integer overflow
  println!("{}", e) // ini akan menjadi 0 karena i8 tidak dapat menampung nilai sebesar i64
}

#[test]
fn numeric_operator() {
  // Ya tau lah [*, /, +, -]
  let a = 10;
  let b = 10;
  let c = a * b;
  println!("{}", c);
  let d = a / b;
  println!("{}", d);
  let f = a + b;
  println!("{}", f);
  let g = a - b;
  println!("{}", g);
}

#[test]
fn augmented_assignments() {
  // Ya tau lah [*=, /=, +=, -=, %=]
  let mut a = 10; // Variable harus mutable, jika immutable akan error
  
  a += 10;
  println!("{}", a);
  
  a -= 10;
  println!("{}", a);
  
  a *= 10;
  println!("{}", a);
  
  a %= 10;
  println!("{}", a);

  a += 10;
  println!("{}", a);

  a /= 10;
  println!("{}", a)
}

#[test]
fn boolean() {
  let a = true;
  let b:bool = false;

  println!("{}, {}", a, b);
}

#[test]
fn comparison_operator() {
  let a = 10;
  let b = 20;

  let mut result = a > b; // Lebih dari
  println!("{}", result);

  result = a < b; // Kurang dari
  println!("{}", result);

  result = a >= b; // Lebih dari sama dengan
  println!("{}", result);

  result = a <= b; // Kurang dari sama dengan
  println!("{}", result);

  result = a == b; // Sama dengan
  println!("{}", result);
}

#[test]
fn boolean_operator() {
  let tugas1 = 49;
  let uts = 80;
  let tugas2 = 90;
  let uas = 80;
  let lulus_tugas = 70;
  let lulus_ujian = 70;

  let lulus_nilai_tugas = (tugas1 + tugas2) / 2 >= lulus_tugas;
  println!("{}", lulus_nilai_tugas);

  let lulus_nilai_ujian = (uts + uas) / 2 >= lulus_ujian;
  println!("{}", lulus_nilai_ujian);

  let sekolah_standard_tinggi = lulus_nilai_tugas && lulus_nilai_ujian;
  println!("{}", sekolah_standard_tinggi);

  println!("{}", !sekolah_standard_tinggi); // true

  let sekolah_standard_mid = lulus_nilai_tugas || lulus_nilai_ujian;
  println!("{}", sekolah_standard_mid);
}

#[test]
fn char_type() {
    let a = 'c';
    let b = 'o';
    let c = 'k';
    println!("{}, {}, {}", a, b, c)
}

// Tipe data compound

/*
Tuple :
Data pada tuple itu final, yang berarti  tidak bisa berkurang atau bertambah.
*/

#[test]
fn tuple() {
  let data: (i8, char, bool) = (10, 'w', false);
  println!("{:?}", data);

  let a = data.0; // Cara mengakses data tuple satu per satu
  let b = data.1;

  println!("{} {}", a, b)
}

#[test]
fn destructuring_tuple() {
  let data: (i8, char, bool) = (10, 'w', false);
  // Atau menggunakan Destructuring tuple (membongkar dan menyimpan ke variable)
  let (a, _, c) = data; // mengakses seluruh data tuple  

  println!("{} {}", a, c)
}

#[test]
fn mutable_tuple() {
  let mut data: (i8, char, bool) = (10, 'w', false); // data tuple dapat di ubah, tetapi tipe data tidak
  println!("{:?}", data);

  data.0 = 120;
  data.1 = 'd';

  println!("{:?}", data)
}

// Di rust function dapat mengembalikan tuple kosong
#[cfg(test)] // untuk menandai bahwa fungsi ini digunakan untuk testing
fn unit() { // ini mereturn tuple kosong
    println!("Hello")
}

#[test]
fn test_tuple_kosong() {
    let result: () = unit(); // ini tipe data tuple kosong

    println!("{:?}", result);

    let test: () = (); // unik jir
    println!("{:?}", test)
}

