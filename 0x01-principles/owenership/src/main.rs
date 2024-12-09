fn main() {
    // demonstrating moves & copy
    let x = vec!["Tyler".to_string()];
    let y = x.clone();
    let z = y.clone();
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    //demonstrating moves
    let s = String::from("takes");
    takes_ownership(s); //move
                        // println!("{}", s); //can't call s again because its borrowed
    let val = 1;
    make_copy(val); //copy
                    //println!("{:?}", val); //this works because val is copied
    let str1: String = give_ownership();
    println!("{}", str1);

    let str2: String = take_n_give(str1);
    println!("{}", str2);

    // references and borrowing
    let mut s = String::from("hello");
    change_string(&mut s);
    println!("{}", s);


}

fn change_string(some_str: &mut String){
    some_str.push_str(", world!");
}
fn takes_ownership(s: String) {
    let strin = s;
    println!("{}", strin);
}

fn make_copy(one: i32) {
    let valin = one;
    println!("{:?}", valin);
}

fn give_ownership() -> String {
    "given".to_string()
}

fn take_n_give(str: String) -> String {
    str
}
