
//problem DONE 👌
// https://leetcode.com/problems/reverse-words-in-a-string/description/?envType=study-plan-v2&envId=leetcode-75

pub fn reverse_words(s: String) -> String {

   let s: &str =  s.trim();
    let mut res : Vec<String> = vec![];
    let s :Vec<char>= s.chars().collect();
    

    let mut loc = 0usize;
   

    while loc < s.len() {
        
        let mut tmp:String= String::default();

        while !s[loc].is_whitespace(){
            tmp.push(s[loc]);
            loc+=1;
            
            if loc >= s.len() { break;}
        }
           if !tmp.is_empty() { res.push(tmp); }
            loc+=1;
    }

res.reverse();

res.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = reverse_words("F R  I   E    N     D      S      ".into());
        assert_eq!(result, "S D N E I R F");
    }
}
