pub fn bubble(arr: &mut [i32]){
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[j] < arr[i] {
                arr.swap(j,i);
            }
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

}

 fn main() {}
