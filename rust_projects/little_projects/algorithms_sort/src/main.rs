pub fn bubble(arr: &mut [i32]){
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[j] < arr[i] {
                arr.swap(j,i);
            }
        }
    }
}

pub fn insertion(arr: &mut [i32]){
    for i in 0..arr.len() {
        for j in (0..i).rev() {
            if arr[j] > arr[i] {
                arr.swap(j,i);
            }
        }
    }
}

pub fn selection(arr: &mut [i32]){
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i..arr.len() {
            if arr[j] < arr[i] {
                min_index = j;
            }
        }
        if i != min_index {
            arr.swap(i,min_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_test(){
        let mut arr = [1,2,3,4,6,5];
        bubble(&mut arr);
        assert_eq!([1,2,3,4,5,6], arr);
    }

    #[test]
    fn insertion_test(){
        let mut arr = [1,2,3,4,6,5];
        insertion(&mut arr);
        assert_eq!([1,2,3,4,5,6], arr);
    }

    #[test]
    fn selection_test(){
        let mut arr = [1,2,3,4,6,5];
        selection(&mut arr);
        assert_eq!([1,2,3,4,5,6], arr);
    }
}

 fn main() {}
