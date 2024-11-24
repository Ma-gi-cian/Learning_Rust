fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is {x}"); // output : 12
    }

    println!("The value of x is {x}"); // output : 6
}
// This is shadowing. we need to use the let keyword or else it will
// be seen as trying to change an already existing immutable variable.
// Everytime we are creating a new variable just with the same name.
/* 
Like variables constants are also used for storing the values.
- cant use mut with constant : declared as const foo = "bar";
- the type of the value must be anotated


let mut x = 5; // Its mutable
println!("The value of Current X is {x}");
x = 6;
println!("The value of x is {x}");
*/

