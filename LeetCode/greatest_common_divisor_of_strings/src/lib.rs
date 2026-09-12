// Problem -> DONE👌
// solve key is eclidiam algorithms
// https://leetcode.com/problems/greatest-common-divisor-of-strings/description/?envType=study-plan-v2&envId=leetcode-75

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    
    let mut str_test_1:String = String::with_capacity(str1.len() + str2.len() );
    str_test_1.push_str(&str1); str_test_1.push_str(&str2);
    let mut str_test_2:String = String::with_capacity(str1.len() + str2.len() );
    str_test_2.push_str(&str2); str_test_2.push_str(&str1);

     if !str_test_1.eq(&str_test_2){
     return String::new();
     }

     let gcd = |a: usize, b: usize| -> usize {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    };
    let (a , b ) ={
        if str1.len() > str2.len(){
            (str1.len(), str2.len())
        }else{
            (str2.len(), str1.len())
        }
    } ;
        str1[0..gcd(a, b)].to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = gcd_of_strings("Ayman".into(), "AymanAymanAyman".into());
        assert_eq!(result, "Ayman");
    }
}
