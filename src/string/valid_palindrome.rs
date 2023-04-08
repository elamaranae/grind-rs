/*
   A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and
   removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric
   characters include letters and numbers.

   Given a string s, return true if it is a palindrome, or false otherwise. 
*/

pub fn run(s: String) -> bool {
    let mut iter = s.chars().filter_map(|c| {
        if c.is_alphanumeric() { Some(c.to_ascii_lowercase()) } else { None }
    });

    loop {
        let left = iter.next();
        let right = iter.next_back();

        match (left, right) {
            (Some(l), Some(r)) => {
                if l != r { return false; };
            },
            (Some(_) | None, None) => break,
            _ => return false
        }
    }

    true
}

#[test]
fn case_0() {
    assert!(run(String::from("A man, a plan, a canal: Panama")));
}

#[test]
fn case_1() {
    assert!(!run(String::from("race a car")));
}

#[test]
fn case_2() {
    assert!(run(String::from(" ")));
}

#[test]
fn case_3() {
    assert!(run(String::from("dad")));
}
