fn main() {
    println!("Hello, world!");

    println!("Hello Eko");

    println!("Hello Budi");
}

#[test]
fn hello_test(){
    println!("Hello Test");
}

#[test]
fn test_variable(){
    let name = "Eko Kurniawan Khannedy";
    println!("Hello {}", name);
}

#[test]
fn test_mutable(){
    let mut name = "Eko Kurniawan Khannedy";
    println!("Hello {}", name);

    name = "Budi Nugraha";
    println!("Hello {}", name);
}

#[test]
fn static_typing(){
    let name = "Eko Kurniawan Khannedy";
    println!("Hello {}", name);

    // name = 10;
    println!("Hello {}", name);
}

#[test]
fn shadowing(){
    let name = "Eko Kurniawan Khannedy";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

/*
   ini komentar lebih dari satu baris
   ini komentar lebih dari satu baris
   ini komentar lebih dari satu baris
   ini komentar lebih dari satu baris
 */
#[test]
fn comment(){
    // ini komentar
    println!("Hello"); // ini komentar lagi
}