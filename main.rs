/*A Practical number P has some selection of its proper divisors, (other than itself),
that can be selected to sum to every integer less than itself.
Compute all the proper divisors/factors of an input number X, then, using all selections
from the factors compute all possible sums of factors and see if all numbers from
1 to X-1 can be created from it.
Task
Write a function that given X returns a boolean value of whether X is a Practical number, (using the above method).
-Show how many Practical numbers there are in the range 1..333, inclusive.
-Show that the Practical numbers in the above range start and end in:
1, 2, 4, 6, 8, 12, 16, 18, 20, 24 ... 288, 294, 300, 304, 306, 308, 312, 320, 324, 330
Stretch Goal
-Show if 666 is a Practical number*/

fn get_factors(x: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    for i in 1..x {
        if x % i == 0 {
            factors.push(i);
        }
    }
    return factors;
}

fn is_practical_number(x: i32) -> bool {
    let factors = get_factors(x);
    let mut sums = Vec::new();
    for i in 0..factors.len() {
        let mut temp = Vec::new();
        for j in 0..factors.len() {
            if i != j {
                temp.push(factors[j]);
            }
        }
        sums.push(temp.iter().sum());
    }
    for i in 1..x {
        if !sums.contains(&i) {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("{}", is_practical_number(6));
}