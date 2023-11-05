fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let mut arr = vec![4, 2, 9, 6, 1, 5, 3];
    bubble_sort(&mut arr);
    println!("{:?}", arr);

    let mut arr2 = vec!['b', 'a', 'd', 'c', 'f', 'e', 'g'];
    bubble_sort(&mut arr2);
    println!("{:?}", arr2);

    let mut arr3 = vec!["apple", "banana", "cherry", "durian"];
    bubble_sort(&mut arr3);
    println!("{:?}", arr3);
}
