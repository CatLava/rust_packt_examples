use std::fmt::Debug;
// Big O notation, run complexity of an algorithm
// O(n^2) This is a measure of how much
pub fn bubble_sort<T:PartialOrd + Debug> (v:&mut [T]) {
    for p in 0..v.len(){
        println!("{:?}",v);
        let mut sorted = true;
        for i in 0..(v.len()-1) - p{
            if v[i] > v[i+1] {
                v.swap(i, i+1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

// This is a recursive program, essentially keeps running and looping over itself
// this is the assumption,
pub fn merge_sort<T:PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // sort the left half,
    // sort the right half, O(n * ln(n))
    // bring sorted halves together. O(n)
    if v.len() <= 1 {
        return v;
    }
    let mut result = Vec::with_capacity(v.len());
    // split off function will split these two
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    // bring them together again
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    // get first elements from list
    let mut a_peak = a_it.next();
    let mut b_peak = b_it.next();

    loop {
        match a_peak {
            Some(ref a_val) => match b_peak {
                Some(ref b_val) => {
                    if b_val < a_val {
                        result.push(b_peak.take().unwrap());
                        b_peak = b_it.next();
                    } else {
                        result.push(a_peak.take().unwrap());
                        a_peak = a_it.next();
                    }
                }
                None => {
                    result.push(a_peak.take().unwrap());
                    result.extend(a_it);
                    return result;
                }
            },
            None => {
                if let Some(b_val) = b_peak {
                    result.push(b_val);
                }
                result.extend(b_it)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![5,8,2,10];
        bubble_sort(&mut v);
        assert_eq!(v, vec![2,5,8,10]);
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![5,8,2,10];
        let v = merge_sort(v);
        assert_eq!(v, vec![2,5,8,10]);
    }
}
