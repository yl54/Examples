// This is an example rust application.

fn main() {
    println!("print_hello()");
    print_hello();
    println!();

    println!("print_number()");
    print_number();
    println!();

    println!("print_variable_addition()");
    print_variable_addition();
    println!();

    println!("test_bool()");
    test_bool();
    println!();

    println!("test_tuple()");
    test_tuple();
    println!();

    println!("test_array()");
    test_array();
    println!();

    println!("test_slice()");
    test_slice();
    println!();

    println!("test_unused_variables()");
    test_unused_variables();
    println!();

    println!("test_scoping()");
    test_scoping();
    println!();

    println!("test_expressions()");
    test_expressions();
    println!();
}

fn print_hello() {
    println!("Hello");
}

fn print_number() {
    println!("{} + {} = {}", 1, 2, 1 + 2);
}

fn print_variable_addition() {
    // These are all immutable variables.
    let num_1: i16 = 32;
    let num_2: i16 = 18;
    let num_3: i16 = num_1 + num_2;
    println!("{} + {} = {}", num_1, num_2, num_3);

    // num_4 is a mutable variable.
    // Note: Must use an assignment operation in a code path.
    // Lesson: cannot have useless assignments.
    let mut num_4: i16 = num_1 + num_2;
    num_4 += 12;
    println!("{} + {} = {}", num_1, num_2, num_4);
}

/*
// Note: Cannot have unused functions.
fn empty_function() {

}
*/

// Note: Can do this for hex and shifts and bitwise operations. 
//       These can be superfast ways to quickly adjust numbers.
fn test_bool() {
    // Make a few bool variables.
    let bool_1: bool = true;
    let bool_2: bool = false;
    let mut bool_3 = true;
    bool_3 = bool_3 ^ false;

    // Print out the boolean results.
    println!("{} && {} = {}", bool_1, bool_2, bool_1 && bool_2);
    println!("bool_3 ^ false = {}", bool_3);
}

fn test_tuple() {
    // Make a tuple.
    let tuple = (1, 2, 3, 4, 5);
    let tuple_value: i16 = tuple.1;

    // Try some manipulations of tuples.
    let sum: i16 = tuple.2 + tuple.4;

    // Print out the tuple stuff.
    println!("tuple = {:?}", tuple);
    println!("tuple_value = {}", tuple_value);
    println!("sum = {}", sum);
}

fn test_array() {
    // Make some array of numbers.
    let arr_1: [i16; 4] = [0, 1, 2, 3];
    let arr_2: [i16; 3] = [4, 5, 6];

    // Combine a few numbers.
    let num_1: i16 = arr_1[3] + arr_2[0];

    // This conversion does not work.
    // let num_2: i32 = arr_2[0] + arr_2[2];

    // Print out some results.
    println!("num_1 = {}", num_1);
}

fn test_slice() {
    // Make an array.
    let arr: [i16; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Make a slice refer to the array.
    // This slice gets [3, 4, 5, 6.]
    let slice: &[i16] = &arr[3 .. 7];

    // Loop over the slice to get a sum.
    let mut sum: i16 = 0;
    for num in slice {
        sum += num;
    }

    // Print out some results from this slice.
    println!("sum = {}", sum);
}

fn test_unused_variables() {
    // Can silence the unused variable with an underscore at beginning of variable name.
    let _unused_num: i16 = 4;
}

fn test_scoping() {
    let outside_var: i16 = 30;

    // inside_var and inside_sum are within this scope
    {
        let inside_var: i16 = 10;
        let inside_sum: i16 = outside_var + inside_var;
        println!("{} + {} = {}", outside_var, inside_var, inside_sum);
    }

    println!("{}", outside_var);
}

