fn main() {

    let mut sum = 0;
    let mut n_minus_one = 1;
    let mut n_minus_two = 0;
    loop {
      let tmp = n_minus_one;
      n_minus_one += n_minus_two;
      n_minus_two = tmp;

      if n_minus_one % 2 == 0 {
        sum += n_minus_one;
      }

      if n_minus_one > 4_000_000 {

        println!("sum {}", sum);
        break;
      }

    }

}
