// fn main() {
//     // variabel yang bersifat imutable
//     let name = "Mhd Fadhil arya putra";
//     println!("{}", name);

//     // variabel yang bersifat umtabke / variabel yang bisa di ubah
//     let mut  name2 = "padhil";
//     println!("Hello, world!");

// }


// shadowing
// fn main() {

//     let x = 1;
//     let x = x * 2;

//     println!("{}", x);
// }


// dangling reference
// fn main()  {
//     let s;
 
//        {
//        let x = 5;
//        s = &x;

//     }

//   println!("{}", s);

// }

// liftime in function
// fn main() {
    
//     let first_name = "Mhd Fadhil";
//     let last_name = "Arya Putra";

//     let longest = longest(first_name, last_name);

//     print!("{}", longest);
// }
// fn longest<'a>(first: &'a str, second: &'a str) -> &'a str{

//      if first.len() > second.len() {
//        first
//     }else {
//       second
//     }
// }

// tipe data collections

// // vector
// fn main() {
  
//     let mut numbers: Vec<i32> = Vec::new();
//        numbers.push(1);

//        match numbers.get(0) {
//            Some(v) => println!("{}", v),
//            None => println!("index not found!"),

//        }
       
//     // menginiliasisasi standart nilai default
//    // let names: Vec<i32> = vec![1, 2, 3];

//    // dengan menggunakan cara refrence
//      println!("{}", numbers[0]);

//       // dengan cara menggunakan looping
//     //   for number in numbers {
//     //    println!("{}", number);
//     // }

//      // dengan mutable reference
//      for number in &mut numbers{
//       *number += 10;
//       println!("{}", number);
//     }

//      // string
//      let fullname = "Mhd Fadhil Arya pUtra";
//      let convert_name_tostring = fullname.to_string();
//      println!("{}", convert_name_tostring);
     
//      // membuat string dengan nilai bawan atau biasa di sebut dengan initiial content
//      let mut say_hello = String::from("Hello");
//       say_hello.push_str(" padhil");
//       println!("{}", say_hello);


//       // menggabungkan data dengan  dengan menggunakan function format

//       let first_name = "Mhd Padhil";
//       let last_name = "Arya Putra";
//       let full_name = format!("{} {}", first_name, last_name);
//       println!("{}", full_name);

// }

use std::collections::HashMap;


fn main(){

  let mut nilai = HashMap::new();
  nilai.insert(String::from("Pemrograman Rust"), 100);
  nilai.insert(String::from("Pemrograman Js"), 80);

  // mengakesek value has map
    println!("{:?}", nilai.get(&String::from("Pemrograman Rust")));
 // println!("{:?}", nilai);

  // buat default nilai pada key jika nilai ny tidak ada
  nilai.entry(String::from("Pemrograman Js")).or_insert(85);
  nilai.entry(String::from("Pemrograman Java")).or_insert(100);
  println!("{:?}", nilai);

   // memecah sebuah string pada white space
   let a = "nama saya adalah muhammad fadhil arya putra";

   let mut result = HashMap::new();

    for b in a.split_whitespace(){
      let count = result.entry(b).or_insert(0);
      *count += 1;
    }
    println!("{:?}", result);
}