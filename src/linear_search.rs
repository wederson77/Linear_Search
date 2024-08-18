pub fn linear_search(vector: &Vec<i32>, value: i32) -> i32{
    for (index, &val) in vector.iter().enumerate(){
        if val == value{
            return index as i32;
        }
    }

    -1
}