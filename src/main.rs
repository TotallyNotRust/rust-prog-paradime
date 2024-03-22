#![allow(unconditional_recursion)]
#![allow(dead_code)]
use std::slice::Iter;
use std::ops::{Mul};
use contracts::*;


fn main() {
    // let time_5 = times(5);
    // let map = curry_map_rec(vec![1,2,3], time_5);
    // println!("{:#?}", map);

    contracts_stuff(2, 9);
    contracts_stuff(2, 2);
    contracts_stuff(2, 7);
}

fn factorial(n:usize) -> usize {
    (1..=n).reduce(|x,y|x*y).unwrap()
}
fn factorial_rec(n:i32) ->i32 {
    fn _factorial_rec(n: i32, c: i32) -> i32 {
        _factorial_rec(n-1, c*n)
    }
    
    _factorial_rec(n, 0)
}
fn fibbo(n:i32) -> i32 {
    fn internal_fibbo(n: i32, a: i32, b: i32) -> i32 {
        match n {
            0 => b,
            _ => internal_fibbo(n-1, b, a+b)
        }
    }
    match n {
        i32::MIN..=2 => 1,
        _ => internal_fibbo (n-1, 0, 1)
    }
}

fn curry_map<A: Mul<A, Output = A>>(lst: Vec<A>, func: impl Fn(A) -> A) -> Vec<A> {
    lst.into_iter().map(func).collect()
}

fn curry_map_rec<A: Mul<A, Output = A> + Clone>(mut lst: Vec<A>, func: impl Fn(A) -> A) -> Vec<A> {
    fn _curry_map_rec<A: Mul<A, Output = A> + Clone>(lst: &mut [A], func: impl Fn(A) -> A, iter: usize) -> &mut [A] {
        if iter < lst.len() {
            lst[iter] = func(lst[iter].clone());
            return _curry_map_rec(lst, func, iter + 1);
        }
        else {
            return lst;
        }
    }

    _curry_map_rec(&mut lst, func, 0).to_vec()
}
fn times <A: Mul<A, Output = A> + Copy>(a: A) -> impl Fn(A) -> A {
    move |b| -> A {
        a * b
    }
}

#[requires(x > 0, "no valid")]
#[requires(y > 0, "no valid")]
#[ensures(x * y == ret, "no valid")]
fn contracts_stuff(x: i32, y: i32) -> i32 {
    x * y
}