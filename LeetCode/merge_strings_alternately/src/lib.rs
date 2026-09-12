// Problem -> DONE 👌
// https://leetcode.com/problems/merge-strings-alternately/description/?envType=study-plan-v2&envId=leetcode-75


pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result:String = String::new();
    let chars = word1.chars().zip(word2.chars());

    for i in chars{
        result.push(i.0); result.push(i.1);
    }

    if word1.len() > word2.len(){
       result.push_str(&word1[word2.len()..] );
    }else{
        result.push_str(&word2[word1.len()..] );
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = merge_alternately(String::from("135"), String::from("24") );
        assert_eq!(result, String::from("12345"));
    }
}


// 