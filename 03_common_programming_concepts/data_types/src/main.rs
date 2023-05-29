fn main() {

    //Numeric operations
    let sum = 5 + 10;
    println!("{sum}");

    let difference = 95.4 - 4.3;
    println!("{difference}");

    let product = 4 * 30;
    println!("{product}");

    let quotient = 56.7 /32.2;
    let floored = 2 / 3;
    println!("{quotient}");
    println!("{floored}");

    let remainder = 43 % 5;
    println!("{remainder}");

    //The Boolean Type
    let _t = true;
    let _f: bool = false;

    //The Charater Type
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}",c,z,heart_eyed_cat);

    //Compound Types - tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {y}");
    println!("The value of z is: {}", tup.2);

    //Compound Types - array type
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let january = months[0];
    let december = months[11];
    println!("First year's month is {january}");
    println!("Last year's month is {december}");

    //declares an array with 4 copies of the value "copy"
    let _copies = ["copy"; 4];
}
