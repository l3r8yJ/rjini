mod ops;

/// `Jini` is a struct that has a single field called `body` that is a `String`.
///
/// Properties:
///
/// * `body`: The body of the Jini.
pub struct RJini {
    pub body: String,
}

/// It's defining a function called `empty` that returns a new `Jini` with an empty body.
impl RJini {
    /// `empty()` returns a `Jini` struct with an empty `body` field
    ///
    /// Returns:
    ///
    /// A Jini struct with an empty body.
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let j = RJini::empty();
    /// assert_eq!("", j.body)
    /// ```
    pub fn empty() -> Self {
        RJini {
            body: "".to_string(),
        }
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn check_creates_empty_rjini() -> Result<()> {
    let j = RJini::empty();
    assert_eq!("", j.body);
    Ok(())
}
