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
    println!("{:?}", test);
}

// Tipe data array di rust tidak bisa dinamis,
// Jadi array di rust cuma bisa 1 tipe data saja.
#[test]
fn array() {
    // Implisit
    let array1 = [1,2,3,4,5];
    // Explisit
    let array2: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", array1);
    println!("{:?}", array2);
}

#[test]
fn cara_akses_array() {
    let array = [1,2,3,4,5];

    let a = array[0]; // cara mengakses array bisa menyebutkan 
    let b = array[3]; // "variable[array yang ingin di akses]"

    println!("{} {}", a, b)
}

#[test]
fn mutable_array() {
    let mut array = [1,2,3,4,5];
    println!("{:?}", array);

    let a = array[0]; // cara mengakses array bisa menyebutkan 
    let b = array[3]; // "variable[array yang ingin di akses]"

    println!("{} {}", a, b);

    array[0] = 10; // cara mengganti nilai array
    array[4] = 50; // dengan mutable

    println!("{:?}", array);
}

#[test]
fn len_array() {
    let array = [1,2,3,4,5];
    println!("{:?}", array);

    println!("{}", array.len());// cara mengetahui panjang array bisa dengan fungsi bawaan "len()"
}

#[test]
fn dimensional_array() {
    // 3 adalah isi array
    // 2 pertama adalah baris array
    // 2 kedua adalah elemen
    let matrix: [[[i32; 3]; 2]; 2] = [ 
      [
        [1, 2, 3],
        [4, 5, 6],
      ],
      [
        [7, 8, 9],
        [10, 11, 12]
      ],
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][0][0]);
    println!("{:?}", matrix[0][0][1]);
    println!("{:?}", matrix[0][0][2]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][1][0]);
    println!("{:?}", matrix[0][1][1]);
    println!("{:?}", matrix[0][1][2]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][0][0]);
    println!("{:?}", matrix[1][0][1]);
    println!("{:?}", matrix[1][0][2]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][1][0]);
    println!("{:?}", matrix[1][1][1]);
    println!("{:?}", matrix[1][1][2]); 
}

// variable const
#[cfg(test)] // untuk menandai bahwa fungsi ini digunakan untuk testing
const MAXIMUM: i32 = 800;

#[test]
fn constant() {
    // Untuk menamai variable const harus menggunakan uppercase semua
    // Tipe data harus disebutkan
    // Nilai harus langsung disebutkan
    const MINIMUM: i64 = 9000;

    println!("{}, {}", MINIMUM, MAXIMUM)
}

// Scope
// Jika variable sejajar dengan function itu akan bisa di akses
// serta juga variable yang sejajar dengan scope itu bisa di akses dari dalam scope.
// Jika sudah diluar scope maka atau variable tidak sejajar dengan function maka tidak bisa di akses.

#[cfg(test)] // untuk menandai bahwa fungsi ini digunakan untuk testing
const PRINT: &str = "Print"; // ini bisa di akses dari function manapun karena ini diluar scope dan funtion
#[test]
fn variable_scope() {
    println!("{}", PRINT); // Contoh di akses di fungsi ini atau fungsi diatas lainnya

    let number = 1;

    {// inner scope 
      println!("{}", number); // number bisa di kases karena variable di outer scope
      println!("{}", PRINT); // Serta variable terluar bisa di akses karena ini juga outer scope

      let number2 = 2;
      println!("{}", number2); // ini bisa di akses karena masih didalam scope yang sama
    }

    // kalau mencoba mengakses variable di inner scope maka akan error 
    // karena variable yang di inner scope tidak bisa di akses di outer scope
    // println!("{}", number2); // ini tidak bisa di akses karena sudah di outer

}

// Contoh visualisasi: pada learn_rust.excalidraw
#[test]
fn memory_management_stack_heap() {
   test_a();
   test_b();
}

#[cfg(test)]
fn test_a() {
    let a = 10;
    let b = String::from("femboy jawa");

    println!("{} {}", a, b)
}

#[cfg(test)]
fn test_b() {
    let a = 10;
    let b = String::from("femboy sunda");

    println!("{} {}", a, b)
}

/*
&str (string slice)
tipe data ini adalah tipe data yang ukurannya fixed size yang
artinya data ini akan disimpan di stack
karena default rust itu immutable maka berlaku juga di &str
jadi ketika kita membuat variable dengan tipe &str 
lalu set ke mutable maka ini mengganti data variablenya, 
bukan isi dari &str. 

misal 
let mut x = 10;
x = 20;

ini akan mengganti panah yang awalnya menunjuk x ke 10
berganti ke x = 20

&str memiliki method untuk memanipulasi &str itu sendiri
tetapi hasilnya adalah nilai &str baru

tetapi kadang return/kembalian dari method &str adalah String
yang disimpan di heap
*/

#[test]
fn string_slice() {
    let name = "                   yantop           ";
    let trim_name = name.trim();

    println!("{}", name);
    println!("{}", trim_name);
}

/*
String
- String di rust adalah tipe data teks UTF-8, 
  kalau di set mutable bisa berkembang (heap)
- String juga bisa dibuat Immutable, 
  jadi tidak bisa berkembang (tetap disimpan di heap)
- String juga memiliki method, tetapi perlu diperhatikan
  method yang kita gunakan apakah memodifikasi mengembangkan
  data mengubah datanya sendiri, atau menduplikat data 
  (tampung ke variable baru)

Contoh visualisasi: pada learn_rust.excalidraw
*/

#[test]
fn string() {
    // let name = String::from("yantop widodo"); // immutable
    let mut name = String::from("yantop widodo"); // harus mutable agar bisa berkembang
    println!("{}", name);

    name.push_str(" subianto");
    println!("{}", name);

    let name2 = name.replace("yantop", "jokowi");
    println!("{}", name2) // hati hati replace mengembalikan data baru/duplikat
}

/*
OWNERSHIP RULES
- Setiap value harus punya owner (variable pemilik value)
- Dalam satu waktu, data hanya boleh ada 1 owner saja
- Ketika owner keluar scope, value akan dihapus
*/

#[test]
fn ownership_rules() {
    let a = 10; println!("{}", a);// sudah disimpan di stack
    // variable a bisa di akses mulai dari sini, 
    // jika mencoba mengakses diatasnya akan error
    {// variable b juga dapat di akses mulai dari line yang sama atau dibawahnya
      let b = 20; // sudah disimpan di stack
      println!("{}", b)
    }// di sini scope b sudah selesai, variable b akan dihapus beserta datanya dan tidak bisa di akses lagi

    println!("{}", a);
}// disini scope a sudah selesai, variable a akan dihapus beserta datanya dan tidak bisa di akses lagi

/*
DATA COPY
- Ketika kita berinteraksi dengan data pada variable, data itu dimiliki 1 owner saja
  atau satu variable saja pemiliknya.
- Data yang fixed size (disimpan di stack), ketika kita menambahkan ke variable baru atau
  berbeda (harapannya owner baru), maka hasilnya adalah data akan di copy ke variable yanng
  baru atau variable yang berbeda, sehingga variable yang berbeda/baru (owner baru) hanya
  memiliki data hasil copy variable lama (owner lama)

Contoh visualisasi: pada learn_rust.excalidraw
*/

#[test]
fn data_copy_stack() {
    let a = 10;
    let mut b = a; // copy dari a

    b += 10;

    println!("{} {}", a, b)
}

/*
OWNERSHIP MOVEMENT HEAP
- Data copy tidak terjadi untuk data yang disimpan di heap
- Ketika kita mencoba membuat variable baru (owner baru) dari variable lama, 
  maka kepemilikan berpindah atau ditransfer dari owner lama ke owner baru, 
  jadi bukan copy.
- Setelah transfer selesai, maka owner lama tidak valid digunakan
*/

#[test]
fn ownership_movement_heap() {
    let mut name1 = String::from("femboy jawa");

    let name2 = name1; // ownership movement

    name1 = String::from("femboy jawa kudus"); // membuat binding baru ke data baru

    println!("{}", name1); // binding ke data baru bisa digunakan kembali
    println!("{}", name2);
}

#[test]
fn ownership_clone() {
    // heap tidak bisa copy data tapi bisa clone
    let name1 = String::from("femboy sunda");
    let name2 = name1.clone(); // clone isi data dari name 1

    println!("{} {}", name1, name2)
}