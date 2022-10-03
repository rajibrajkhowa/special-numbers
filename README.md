This is a non-refactored simple program to determine if a number is "perfect number" or "abundant number" or "deficient number"

All numbers have divisors (even prime number has two divisor, 1 & itself). The divisors other than the number itself are called proper divisors (for prime number proper divisor is 1)

A number is categorized as "perfect number", if sum of its proper divisors is equal to the number itself. Some examples perfect numbers are: 6, 28, 496, 8128. These numbers are quite interesting and they can obtained by the formula n = 2^(p-1) * (2^p - 1), where p is a prime,and the term (2^p - 1) is a Mersenne Prime. The decpetively, simple formula however hides a mystery underneath as we cannot obtain perfect number by using any prime number "p",as on date only 51 known perfect numbers have been detected

A number is categorized as "abundant number", if sum of its proper divisors is greater than the number itself. Some examples are 20, 30, 12, 18, etc.

A number is categorized as "deficient number", if sum of its proper divisors is less than the number itself. Some examples are all prime numbers are deficient number as they have a single proper divisor 1, all numbers of the form a^n, where "a" and "n" are positive integers (like 2^10, 8^7, 23^8, etc), etc.

Caution: Please avoid entering very big numbers as the program will respond slowly, number theory related programs should use highly optimized and fast libraries, like in C we have pari lib and C++ we have NTL library

Screenshots#

![demo](https://user-images.githubusercontent.com/82666308/193574642-2a0eec8b-d7f4-4e60-b306-00f873eacfae.JPG)
