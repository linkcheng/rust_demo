use std::cmp::PartialOrd;

fn main() {
    println!("================");
    let mut arr= vec![1, 3, 4, 7, 9, 2, 6, 5, 8, 0];
    println!("{:?}", arr);
    bubble_sort_i32(&mut arr);
    println!("{:?}", arr);

    println!("================");
    let mut arr1= vec![1.0, 3.0, 4.0, 7.0, 9.0, 2.0, 6.0, 5.0, 8.0, 0.0];
    println!("{:?}", arr1);
    bubble_sort(&mut arr1);
    println!("{:?}", arr1);

    println!("================");
}


fn bubble_sort_i32(arr: &mut Vec<i32>) {
    bubble_sort(arr)
    // for i in 0..arr.len() {
    //     for j in 0..arr.len() - 1 - i {
    //         if arr[j] > arr[j + 1] {
    //             arr.swap(j, j + 1);
    //         }
    //     }
    // }
}

fn bubble_sort<T: PartialOrd>(arr: &mut Vec<T>) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
