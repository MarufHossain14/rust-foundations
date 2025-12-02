fn main(){
  let mut x = 5;
  println!("The value of x is: {x}");
  x = 6;
  println!("The value of x is: {x}");

}

// example of a constant
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

/*
shadowing example:
shadowing creates a new versiona nd replaces the old one.
the old one gets lost, and the ne wone is a different variable.
fn main() {
    let x = 10;      // first x
    let x = x + 5;   // second x (new variable)
    let x = x * 2;   // third x (new variable)

    println!("{}", x); // prints 30
}
*/

/*
shadowing is different from making a variable as mut because we will get a compile time error if we accidentally try to reassign to this variable without using the let keyword.
by using let we can perform a few transformations on a value but changing its value is not one of them.
*/
