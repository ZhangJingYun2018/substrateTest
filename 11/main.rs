
fn main() {
  let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
  let mut arrs  = ["all", "bsd", "sdcnnj", "cka",  "g", "h", "e",  "j", "i","k","f",];
  bubble_sort(&mut arr);
  bubble_sort(&mut arrs);
  println!("{:?}", arr);
  println!("{:?}", arrs);

}

fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
  let n = arr.len();
  for i in 0..n {
    for j in 0..n - i - 1 {
      if arr[j] > arr[j + 1] {
        arr.swap(j, j + 1);
      }
    }
  }
}
