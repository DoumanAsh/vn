#![allow(unused)]

use rand::Rng;

///Generates random number in range [left...right)
pub fn range<T: rand::distributions::uniform::SampleUniform>(left: T, right: T) -> T {
    let mut rng = rand::thread_rng();

    rng.sample(rand::distributions::Uniform::new(left, right))
}

///Generates random number in range [left...right]
pub fn range_inclusive<T: rand::distributions::uniform::SampleUniform>(left: T, right: T) -> T {
    let mut rng = rand::thread_rng();

    rng.sample(rand::distributions::Uniform::new_inclusive(left, right))
}
