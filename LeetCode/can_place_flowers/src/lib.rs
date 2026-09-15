
// problem done 👌
// https://leetcode.com/problems/can-place-flowers/description/?envType=study-plan-v2&envId=leetcode-75

fn is_none_or_zero( elem:Option<&i32> ) -> bool {
    match elem {
        Some(x) => { 
            *x == 0
        }
        None => true,
    }
}

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let len = flowerbed.len(); let mut n = n; let mut flowerbed = flowerbed;

    if n == 0 {
        return true;
    }
    if is_none_or_zero(flowerbed.get(0) ) && is_none_or_zero(flowerbed.get(1)){
        flowerbed[0] =1;
        n -=1;
    }
    
    let mut i = 1;
    while i < len && n >0 {
        if is_none_or_zero(flowerbed.get(i)) &&
        is_none_or_zero(flowerbed.get(i+1)) &&
        is_none_or_zero(flowerbed.get(i -1)) {
            flowerbed[i] =1;
            n -=1;
            i+=2;
        }else{
        i +=1;
        }

    }
    
    n <= 0

}


#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn it_works() {
        let result = can_place_flowers([1,0,0,0,1,0,0].into(), 2);
       
        assert_eq!(result, true);
    }
}