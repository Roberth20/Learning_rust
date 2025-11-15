//  Patterns can be refutable and irrefutable.
//  
//  When they can match with any possible value passed, they are irrefutable.
//  For example, 'x' is 'let x = 5' matches always and can not fail.
//
//  Patterns that can fail to match are refutable. For example, 
//  'Some(x)' in the expression 'if let Some(x) = a_value' can not match if
//  'a_value' is None.
//
//  In general, 'let' and 'for' statements only accept irrefutable patterns.
fn main() {
    // If we try to compile a let assignment with a refutable expression, we 
    // have a compiler error, but it can be fix it
    let Some(x) = some_option else { return; };
}
