fn example_constant() {
    // CONSTANT
    const PI: f32 = 3.14;
    println!("Constant {}", PI);
}

fn example_shadowing() {
    // SHADOWING
    let x = 5;
    println!("Address first x: {:p}", &x);

    let x = x + 1;
    println!("Address second x: {:p}", &x);
    println!("The value of X first: {}", x);

    {
        let x = x * 2;
        println!("The value of X in scope: {}", x);
        println!("Address third x: {:p}", &x);
    }

    println!("The value of X out scope: {}", x);
}

fn example_scalar() {
    let num = 57u8;
    let num_2 = 1_000_000;
    println!("Number 1: {}", num);
    println!("Number 2: {}", num_2);

    let c = 'z';
    let b = true;

    println!("Character: {c} is {b}");
}

fn example_compound() {
    let tup = (1, 2, 3, 4, 5, "String");
    println!("{}", tup.5);

    let arr: [i32; 3] = [0, 1, 2];

    for i in arr {
        println!("{}", i);
    }

    let y = {
        let x = 4;
        x + 2
    };

    println!("{}", y);
}

fn increase_num(x: i32) -> i32 {
    return x + 1;
}

fn condition() {
    let x = 10;
    let y = 20;
    if x % 2 == 0 && y % 2 == 0 {
        println!("Magic couple")
    }
}

fn loop_type() {
    loop {
        println!("Hello world");
    }
}

fn main() {
    //     example_constant();
    //     example_shadowing();
    //     example_scalar();
    loop_type();
}
