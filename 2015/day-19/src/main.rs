use std::collections::BinaryHeap;

fn main() {
  let mut heap = BinaryHeap::new();
  heap.push(0);
  heap.push(3);
  heap.push(3);
  while let Some(x) = heap.pop() {
    println!("{x}");
  }
}
