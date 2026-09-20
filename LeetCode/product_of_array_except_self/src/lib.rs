    
    //Problem Done 👌
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut num_of_zeros:u8 = 0; 
    let mut loc_of_one_zero: usize = 0;
    let len = nums.len();


    let mut multip = 1;
    
    for (loc , elem) in nums.iter().enumerate(){
        if elem == &0 {
            num_of_zeros +=1;

            if num_of_zeros > 1{
                return vec![0; len];
            }
            loc_of_one_zero = loc;
            continue;
        }
        multip *= *elem;
    }
    let mut res =  vec![0 ; len];

    if num_of_zeros == 1{
        res[loc_of_one_zero] = multip;
        return res;
    }
    for (k, i) in nums.iter().enumerate(){
        res[k]= multip/ i;
    }

    res
    }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = product_except_self(vec![1, 2, 3, 4]);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }
}
