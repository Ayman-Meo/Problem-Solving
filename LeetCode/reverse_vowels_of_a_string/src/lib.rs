
// problem done 👌
// https://leetcode.com/problems/reverse-vowels-of-a-string/?envType=study-plan-v2&envId=leetcode-75


pub fn reverse_vowels(s: String) -> String {
    let mut chars:Vec<char> = s.chars().collect();

    let mut left:usize = 0; let mut right:usize = chars.len() -1;

    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];


    while left < right{
        if vowels.contains(&chars[left]){
            'inner: while right > left {
                if vowels.contains( &chars[right] ){
                    chars.swap(left, right);
                    left+=1;
                    right-=1;
                    break 'inner;
                }
                right-=1;
            }
        }else{
            left+=1;
        }
    }
 chars.iter().collect()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = reverse_vowels("IceCreAm".into());
        assert_eq!(result, "AceCreIm");
    }
}
