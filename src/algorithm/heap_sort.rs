use crate::util::vec::*;

fn heap_sort(v: &mut Vec<i32>) {
    let mut end = v.len() - 1;
    for i in (0..v.len()).rev() {
        sink(i, v, end);
    }

    while end > 0 {
        v.swap(0, end);
        end -= 1;
        sink(0, v, end);
    }
}

fn left(i: usize) -> usize {
    i * 2 + 1
}

fn right(i: usize) -> usize {
    left(i) + 1
}

fn parent(i: usize) -> usize {
    (i - 1) / 2
}

fn sink(i: usize, v: &mut Vec<i32>, end: usize) {
    let left = left(i);
    let right = right(i);
    let mut bigger = i;
    if left <= end && v[left] > v[bigger] {
        bigger = left;
    }
    if right <= end && v[right] > v[bigger] {
        bigger = right;
    }
    if bigger != i {
        v.swap(i, bigger);
        sink(bigger, v, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut shuffled = get_shuffled_vec();
        heap_sort(&mut shuffled);
        assert_eq!(get_sorted_vec(), shuffled);
    }
}
