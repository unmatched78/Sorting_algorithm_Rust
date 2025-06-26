use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;

// Bubble Sort
fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr
}

// Insertion Sort
fn insertion_sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
    arr
}

// Selection Sort
fn selection_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    for i in 0..len {
        let mut min_idx = i;
        for j in (i + 1)..len {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
    arr
}

// Merge Sort
fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    let mid = arr.len() / 2;
    let left = merge_sort(arr[..mid].to_vec());
    let right = merge_sort(arr[mid..].to_vec());
    merge(left, right)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

// Quick Sort
fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    let pivot = arr[arr.len() / 2];
    let mut less = vec![];
    let mut equal = vec![];
    let mut greater = vec![];
    for x in arr {
        if x < pivot {
            less.push(x);
        } else if x == pivot {
            equal.push(x);
        } else {
            greater.push(x);
        }
    }
    let mut result = quick_sort(less);
    result.extend(equal);
    result.extend(quick_sort(greater));
    result
}

// Heap Sort
fn heap_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    // Build heap
    for i in (0..len / 2).rev() {
        heapify(&mut arr, len, i);
    }
    // Extract elements from heap
    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(&mut arr, i, 0);
    }
    arr
}

fn heapify(arr: &mut [i32], n: usize, i: usize) {
    let mut largest = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < n && arr[l] > arr[largest] {
        largest = l;
    }
    if r < n && arr[r] > arr[largest] {
        largest = r;
    }
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn main() {
    let n = 10_000;
    // Generate the same random list for all algorithms
    let mut rng = thread_rng();
    let mut original: Vec<i32> = (0..n).collect();
    original.shuffle(&mut rng);

    let sorts = vec![
        ("Bubble Sort", bubble_sort as fn(Vec<i32>) -> Vec<i32>),
        ("Insertion Sort", insertion_sort),
        ("Selection Sort", selection_sort),
        ("Merge Sort", merge_sort),
        ("Quick Sort", quick_sort),
        ("Heap Sort", heap_sort),
    ];

    let mut handles = vec![];

    for (name, sort_fn) in sorts {
        let arr = original.clone();
        let name = name.to_string();
        handles.push(std::thread::spawn(move || {
            let start = Instant::now();
            sort_fn(arr);
            let elapsed = start.elapsed();
            println!(
                "{:<15}: {:>10} ms",
                name,
                elapsed.as_secs_f64() * 1000.0
            );
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
