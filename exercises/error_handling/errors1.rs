// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

/**
 * Two types of errors in rust
 * 1. Recoverable (File errors, things that can be retried
 *    and fixed - missing file)
 * 2. Nonrecoverable (Symptoms of bugs, trying to access
 *    and array outside of it's bounds) 
 * Rust does not have exceptions. Rather, it has 
 * 1. type Result<T, E> for recoverable errors, and 
 * 2. the panic! macro which stops execution of a program when
 *    it recovers an unrecoverable error
 */

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err(String::from("`name` was empty; it must be nonempty."))
    } else {
        Ok(String::from(format!("Hi! My name is {}", name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
