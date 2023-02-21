/// `Jini` is a struct that has a single field called `body` that is a `String`.
///
/// Properties:
///
/// * `body`: The body of the Jini.
pub struct Jini {
    pub body: String,
}

/// It's defining a function called `empty` that returns a new `Jini` with an empty body.
impl Jini {
    /// `empty()` returns a `Jini` struct with an empty `body` field
    ///
    /// Returns:
    ///
    /// A Jini struct with an empty body.
    ///
    /// For example:
    /// ```
    /// use rjini::jini::Jini;
    /// let j = Jini::empty();
    /// assert_eq!("", j.body)
    /// ```
    pub fn empty() -> Self {
        Jini {
            body: "".to_string(),
        }
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn check_creates_empty_jini() -> Result<()> {
    let j = Jini::empty();
    assert_eq!("", j.body);
    Ok(())
}
