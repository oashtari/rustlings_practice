// iterators4.rs

// I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    
    
    
    // let mut fact = num;
    // let mut result = Vec::new();
    
    // while fact > 0 {
    //     result.push(fact);
    //     fact -=1;
    // };

    // let mut orial = result.iter();
    // println!("ORIAL {:?}", orial);
    // // println!("ORIAL {:?}", orial.next().unwrap());
    // let mut howdy: u64 =  *orial.next().unwrap();
    // println!("FIRST VALUE {}", howdy);
    // while orial.len() > 0 {
    //     howdy *= orial.next().unwrap();
    // }
    // // while orial.next() != None {
    // //     howdy *= orial.next().unwrap();
    // // }
    // // println!("HELLO {:?}", result.iter());
    // println!("HOWDY {:?}", howdy);
    
    // howdy



    (1..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
