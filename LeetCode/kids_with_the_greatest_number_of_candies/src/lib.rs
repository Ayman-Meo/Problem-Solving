 // problem -> DONE 👌
 // https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/?envType=study-plan-v2&envId=leetcode-75
 
 
 
pub fn kids_with_candies(candies: Vec<i32>,
     extra_candies: i32) -> Vec<bool> {

        let mut candies1 = candies.clone() ;
        candies1.sort();
        let last = candies1.last().unwrap().clone();
        let mut result:Vec<bool> = vec![];
        for i in candies{
            if i + extra_candies >= last  {
                result.push(true);
            }else{
                result.push(false);
            }
        }
    result
}
/* Best solusion
   pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().copied().max().unwrap_or_default();

        candies.into_iter().map(|x| x + extra_candies >= max).collect()

        
    }
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = kids_with_candies(vec![2, 3, 5, 1, 3], 3);
        assert_eq!(result, vec![true, true, true, false, true]);
    }
}
