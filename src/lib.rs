mod ops;

/// `Jini` is a struct that has a single field called `body` that is a `String`.
///
/// Properties:
///
/// * `body`: The body of the Jini.
#[derive(Debug)]
pub struct RJini {
    pub xpath: String,
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
    /// assert_eq!("", j.xpath)
    /// ```
    pub fn empty() -> Self {
        RJini {
            xpath: "".to_string(),
        }
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn check_creates_empty_rjini() -> Result<()> {
    let j = RJini::empty();
    assert_eq!("", j.xpath);
    Ok(())
}
