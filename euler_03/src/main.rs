fn main() {

  let num : u64 = 600851475143;
  let largest_possible_factor = (num as f64).sqrt() as u64;

  let factors = (1..largest_possible_factor)
    .filter(|x| num % x == 0)
    .collect::<Vec<_>>();

  // Coming back to this problem after I complete a few more,
  // can figure out with calculator now
  for factor in factors {
      println!("{}", factor);
  }
}
