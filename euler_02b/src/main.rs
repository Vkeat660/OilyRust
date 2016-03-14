struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci{
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
      let new_next = self.curr + self.next;

      self.curr = self.next;
      self.next = new_next;

      Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {

  let sum : u64 = fibonacci()
    .take_while(|&i| i<4_000_000)
    .filter(|&i| i%2 == 0)
    .fold(0, |a,b| a+ b);

  println!("The sum is {}", sum);

}
