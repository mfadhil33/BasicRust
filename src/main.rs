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

// use std::collections::HashMap;


// fn main(){

//   let mut nilai = HashMap::new();
//   nilai.insert(String::from("Pemrograman Rust"), 100);
//   nilai.insert(String::from("Pemrograman Js"), 80);

//   // mengakesek value has map
//     println!("{:?}", nilai.get(&String::from("Pemrograman Rust")));
//  // println!("{:?}", nilai);

//   // buat default nilai pada key jika nilai ny tidak ada
//   nilai.entry(String::from("Pemrograman Js")).or_insert(85);
//   nilai.entry(String::from("Pemrograman Java")).or_insert(100);
//   println!("{:?}", nilai);

//    // memecah sebuah string pada white space
//    let a = "nama saya adalah muhammad fadhil arya putra";

//    let mut result = HashMap::new();

//     for b in a.split_whitespace(){
//       let count = result.entry(b).or_insert(0);
//       *count += 1;
//     }
//     println!("{:?}", result);
// }

// fn main(){
  
//   panic!("panic!!");
// }


// use core::panic;
// use std::{fs::File, io::ErrorKind};

// fn main(){
  
//     File::open("hello.txt").unwrap_or_else(|error| {
    
//     if error.kind() == ErrorKind::NotFound {
//       File::create("hello.txt").unwrap_or_else(|error| {
//         panic!("{:?}", error);
//       })
//     } else {
//         panic!("{:?}", error);
//     }
    
//     });
    

//   // let file =  match file {
//   //     Ok(file) => file,
//   //     Err(error) => {
//   //      panic!("Problem: {:?}", error);
//   //     }
//   // };

// use core::num::dec2flt::float;
// use std::{fs::File, io::ErrorKind, num};

//   // Ok(T);
//   // Err(E);
//  }
// fn main(){


//    let file = File::open("file.txt");
//    let file = match file {
//        Ok(file) => file,
//        Err(error) => {
//        panic!("Problem: {:?}", error);
//       }
//    };
// }

// fn main(){

//    // selain unwrap else
//    File::open("hello.txt").expect("File not found");

//     File::open("hello.txt").unwrap_or_else(|error| {
    
//      if error.kind() == ErrorKind::NotFound {
    
//       File::create("hello.txt").unwrap_or_else(|error| {
      
//         panic!("{:?}", error);
//       })
//     }else {
    
//       panic!("{:?}", error);
//     }
//     });
// }

// fn main(){

//   File::open("hello.txt").expect("file not found");


// }


// generic types & trait
// fn main() {

//    let numbers = vec![1, 2, 3, 4, 5];
//    let mut largest = numbers[0];

//    for number in numbers {
//    if number > largest {
//    largest =  number;
//   }
//   }
//    print!("{}", largest);
// }

// menggunakan non generic type
// fn largest(list: &[i32]) -> i32{
//  let mut largest = list[0];

//    for &number in list {
//     if number > largest {
//      largest = number
//     }
//   }
//   largest
// }

// fn main() {
//   let numbers = vec![1, 2, 3, 4, 5];
//   let result = largest(&numbers);
//   println!("{}", result);
// }


// menggunakan generic type
// fn largest<T>(list: &[T]) -> T {
//      let mut largest = list[0];

//      for &item in list {
//      if item > largest {
//        largest = item
//     }
//     }
//     largest
// }

// fn main() {
//   let numbers = vec![10, 1, 2];
//   let largest_number  = largest(&numbers);
//   println!("{}", largest_number);

//    let worlds = vec!["hello", "padhil"];
//    let largest_words = largest(&worlds);
//    println!("{}", largest_words);
// }

// generic type in struct
// struct Point<T> {
//    x: T,
//    y: T
// }

// fn main() {
//  let integer = Point {x: 1, y:2 };
//   let float  = Point { x: 1.2, y: 2.1};

//   println!("{} {}", integer.x, float.y);
// }

// in enum definition
// enum Result<T, E> {

//   Ok(T),
//   Err(E),
// }

// in method definition 
// struct Point<T>{
//   x: T,
//   y: T
// }

// impl <T> Point <T> {
//     fn print_x(&self) -> &T {
//      &self.x
//     }
// }
// fn main() {

//     let integer = Point {x: 1, y: 2};
//     let float = Point { x: 1.2, y: 2.1};

//     println!("{} {} {}", integer.x, float.y, integer.print_x());
  
// }

// multi generic  types with method 
struct Point <T, U> {
  x: T,
  y: U
}

impl <T, U> Point <T, U > {
    
   fn mixup<V, W>(self, other: Point<V, W>) ->  Point <T, W > {
    Point { 
      
      
     x: self.x,
     y: other.y
    
    }
  }
}

// fn main () {

//   let p1 = Point { x: 1, y:2 };
//   let p2 =  Point { x: "Hello", y: "padhil"};
//   let p3 = p1.mixup(p2);

//     println!("{} {}", p3.x, p3.y);
// }

// traits
trait Summary {

 fn summarize(&self) -> String;

}

struct Article {
  title: String,
  author: String
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} author of {}", self.author, self.title)
    }
}

fn main(){

  let first_article =  Article {
  
     title: String::from("Belajar Bahasa pemrograman  Rust"),
     author: String::from("Rust")
  
  };

    println!("{}", first_article.summarize());


}