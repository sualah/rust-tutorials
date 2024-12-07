fn main() {
    let x: i8 = 10;
    println!("{}", x);

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let byte = b'A';
    println!("{}", byte);


    let my_tuple = (500, "hello", false);

    println!("my tuple index 0 is {}", my_tuple.0);

    let (a, b, c) = my_tuple;

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);

    let mut array = [1, 2, 3];

    println!("{}", array[0]);

    array[0] = 10;

    println!("{}", array[0]);

    println!("{:?}", array);

    let mut my_vec = vec![10, 50, 45];

    my_vec.push(60);
    println!("{:?}", my_vec);
}
