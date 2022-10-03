/// This is a non-refactored simple program to determine if a number is "perfect number" or
/// "abundant number" or "deficient number"
/// 
/// All numbers have divisors (even prime number has two divisor, 1 & itself). The divisors
/// other than the number itself are called proper divisors (for prime number proper divisor is 1)
/// 
/// A number is categorized as "perfect number", if sum of its proper divisors is equal to the
/// number itself. Some examples perfect numbers are - 6, 28, 496, 8128. These numbers are quite
/// interesting and they can obtained by the formula n = 2^(p-1) * (2^p - 1), where p is a prime,
/// and the term (2^p - 1) is a Mersenne Prime. The decpetively, simple formula however hides a
/// mystery underneath as we cannot obtain perfect number by using any prime number "p",as on date
/// only 51 known perfect numbers have been detected
/// 
/// A number is categorized as "abundant number", if sum of its proper divisors is greater than the
/// number itself. Some examples are 20, 30, 12, 18, etc.
/// 
/// A number is categorized as "deficient number", if sum of its proper divisors is less than the
/// number itself. Some examples are all prime numbers are deficient number as they have a single
/// proper divisor 1, all numbers of the form a^n, where "a" and "n" are positive integers
/// (like 2^10, 8^7, 23^8, etc), etc.
/// 
/// Caution: Avoid entering very big numbers as the program will respond slowly, number theory
/// related programs should use highly optimized and fast libraries, like in C we have pari lib
/// and C++ we have NTL library


use std::io;

/// This section implements the finding of divisors of a given number using traits and implementation
/// features of RUST
trait DivisorList {
    fn divisor_list(&self) -> Vec<u64>;
}

impl DivisorList for u64 {
    fn divisor_list(&self) -> Vec<u64> {
        let mut divisor: u64 = 1;
        let mut divisor_list: Vec<u64> = Vec::new();
        while divisor <= *self {
           let x: u64 = *self%divisor;
           if x == 0 {
              divisor_list.push(divisor);
            }
              divisor = divisor+1;
         }
        return divisor_list;
    }
}

/// This section implements the simple alogorithm for finding the characteristic of the numbers based on
/// the list of proper divisors obtained in above implementation. And it it outputs, the type of number
/// perfect number or abundant number or deficient number 
trait WhatType {
    fn what_type(&self) -> String;
}

impl WhatType for u64 {
    fn what_type(&self) -> String {
        let x: Vec<u64> = self.divisor_list();
        let mut sum: u64 = x.iter().sum();
        let last_elem: &u64 = x.last().unwrap();
        sum = sum - (*last_elem);
        if sum == *last_elem {
            return String::from("Perfect Number");
        }
        else if sum > *last_elem {
            return String::from("Abundant Number");
        }
        else {
            return String::from("Deficient Number");
        }
    }
}
fn main() {

    let mut n = String::new();
    println!("Please enter the number:");
    io::stdin().read_line(&mut n).expect("Number not entered");
    let n: u64 = n.trim().parse().expect("Please type a number");
    println!("The number {} is {:?}",n,n.what_type());
}