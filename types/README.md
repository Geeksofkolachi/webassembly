# Scalar Types
A scalar type represents a single value. 
Rust has four primary scalar types:
- integer
- floating-point numbers 
- boolean 
- characters

1. Integer Types
An integer is a number without a fractional component.

2. Floating-Point Types
let y: f32 = 3.0;

3. Boolean Types
let boolean: bool = false; 

4. Character Types
let z = 'ℤ';

# Compound Types
Compound types can group multiple values into one type. 
Rust has two primitive:
- tuples 
- arrays

1. Tuple Type
let tup: (i32, f64, u8) = (500, 6.4, 1);

The variable tup binds to the entire tuple, 
because a tuple is considered a single compound element. 
To get the individual values out of a tuple, 
we can use pattern matching to destructure a tuple value

let gok = ('G', 'O', 'K');
let (x, y, z) = gok;
println!("The value of y is: {}", y);

- we can access a tuple element directly by using a period ( . ) 
followed by the index of the value we want to access.
let gok = ('G', 'O', 'K');
let first_name = gok.0;
let second_name = gok.1;

2. Array Type
- Every element of an array must have the same type
- Arrays in Rust have a fixed length

let a = [1, 2, 3, 4, 5] 
    OR
let a: [i32; 5] = [1, 2, 3, 4, 5];
    [type; number] 