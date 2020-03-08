use crate::util::vec::*;

fn quick_sort(v: &mut Vec<i32>, left: i32, right: i32) {
    if left >= right {
        return;
    }
    let p = partition(v, left, right);
    quick_sort(v, left, p - 1);
    quick_sort(v, p + 1, right);
}

fn partition(v: &mut Vec<i32>, left: i32, right: i32) -> i32 {
    let value = v[right as usize];
    let mut pivot = left - 1;
    for i in left..right {
        let i = i as usize;
        if v[i] <= value {
            pivot += 1;
            v.swap(i, pivot as usize);
        }
    }
    pivot += 1;
    v.swap(pivot as usize, right as usize);
    pivot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut shuffled = get_shuffled_vec();
        let right = shuffled.len() as i32 - 1;
        quick_sort(&mut shuffled, 0, right);
        assert_eq!(get_sorted_vec(), shuffled);
    }
}
