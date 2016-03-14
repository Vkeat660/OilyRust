struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement 'Iterator' for 'Fibonacci'
impl Iterator for Fibonacci {
    type Item = u32;
    // The 'Iterator' trait only requires the 'next' method to be defined. The
    // return type is 'Option<T>', 'None' is returned when the 'Iterator' is
    // over, otherwise the next value is returned wrapped in 'Some'
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        // 'Some' is always returned, this is an infinite value generator
        Some(self.curr)
    }
}

// Returns a fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {

  let sum : u32 = fibonacci()
    .take_while(|&i| i<4_000_000)
    .filter(|&i| i%2 == 0)
    .fold(0, |a,b| a+ b);

  println!("The sum is {}", sum);
}
