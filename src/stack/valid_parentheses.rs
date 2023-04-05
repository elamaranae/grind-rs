/*
   Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if
   the input string is valid.

   An input string is valid if:

   Open brackets must be closed by the same type of brackets. Open brackets must be closed in the
   correct order. Every close bracket has a corresponding open bracket of the same type.
*/

fn closing_pair(opening_pair: char) -> char {
    match opening_pair {
        '{' => '}',
        '(' => ')',
        '[' => ']',
        _ => ' '
    }
}

pub fn run(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '{' | '(' | '[' => stack.push(c),
            '}' | ')' | ']' => {
                if let Some(&top) = stack.last() {
                    if c == closing_pair(top) {
                        stack.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            },
            _ => return false
        }
    }

    stack.is_empty()
}

pub fn is_valid(s: String) -> bool {
    run(&s)
}

#[test]
fn case_0() {
    assert!(run("()[]{}"));
}

#[test]
fn case_1() {
    assert!(!run("(]"));
}

/*
 * 1. For a closing parantheses to be valid, it has to match it's nearest opening pair and the
 *    string between the pair must be valid.
 * 2. In this algorithm, the invariant is stack is empty for a valid string.
*/
