use crate::RJini;
use anyhow::anyhow;
use anyhow::Result;
use std::ops::Add;
use std::panic::Location;

/// Creating a new instance of RJini from a XPATH as string.
impl From<&str> for RJini {
    /// It takes a string and returns a RJini object.
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let j = RJini::from("parent/child");
    /// assert!(j.xpath.contains("t/c"))
    /// ```
    ///
    /// Arguments:
    ///
    /// * `xpath`: The XPath expression to be evaluated.
    ///
    /// Returns:
    ///
    /// A struct with a XPATH field.
    fn from(xpath: &str) -> Self {
        RJini {
            xpath: xpath.to_string(),
        }
    }
}

/// @todo #1 Harder unit tests.
/// We have to make our tests as hard as possible.
impl RJini {
    /// It adds a node to the body of the XPATH.
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let j = RJini::from("parent/")
    ///     .add_node("child").unwrap()
    ///     .add_node("game").unwrap();
    /// assert!(j.xpath.contains("child/game/"))
    /// ```
    ///
    /// Arguments:
    ///
    /// * `node`: The node to add to the XPATH.
    ///
    /// Returns:
    ///
    /// A new RJini object with the new body.
    pub fn add_node(&self, node: &str) -> Result<RJini> {
        validate(node)?;
        let b = self.xpath.clone() + node + "/";
        Ok(RJini { xpath: b })
    }

    /// It removes a node from the XPATH.
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let j = RJini::empty()
    ///     .add_node("parent").unwrap()
    ///     .add_node("child").unwrap()
    ///     .add_node("toy").unwrap();
    /// assert!(j.xpath.contains("child"));
    /// let j = j.remove_node("child");
    /// assert!(!j.xpath.contains("child"))
    /// ```
    ///
    /// Arguments:
    ///
    /// * `node`: The node to remove.
    ///
    /// Returns:
    ///
    /// A new RJini object with the XPATH of the old one but with the node removed.
    pub fn remove_node(&self, node: &str) -> RJini {
        let b = self.xpath.replace(&node.to_string().add("/"), "");
        RJini { xpath: b }
    }

    /// > Replace a node in the xpath with a new node
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let j = RJini::empty()
    ///         .add_node("Ruby").unwrap()
    ///         .add_node("is").unwrap()
    ///         .add_node("a").unwrap()
    ///         .add_node("bad").unwrap()
    ///         .add_node("dog").unwrap()
    ///         .replace_node("bad", "good").unwrap()
    ///         .xpath;
    /// assert_eq!("Ruby/is/a/good/dog/", j);
    /// ```
    ///
    /// Arguments:
    ///
    /// * `origin`: The node you want to replace.
    /// * `new`: The new node name
    ///
    /// Returns:
    ///
    /// A new RJini object with the xpath replaced.
    pub fn replace_node(&self, origin: &str, new: &str) -> Result<RJini> {
        validate(new)?;
        let x = self.xpath.replace(origin, new);
        Ok(RJini { xpath: x })
    }
}

/// It checks if the node contains spaces.
///
/// Arguments:
///
/// * `node`: The name of the node to add.
///
/// Returns:
///
/// Result<()>
fn validate(node: &str) -> Result<()> {
    let location = Location::caller();
    if node.contains(' ') {
        return Err(anyhow!(format!(
            "{location}: The \"{node}\" contain spaces"
        )));
    }
    Ok(())
}

#[test]
fn checks_creates_rjini_from() -> Result<()> {
    let j = RJini::from("parent/child");
    assert!(j.xpath.contains("child"));
    Ok(())
}

#[test]
fn checks_adds_node() -> Result<()> {
    let j = RJini::from("parent/");
    let j = j.add_node("child")?;
    let j = j.add_node("toys")?;
    println!("{}", j.xpath);
    assert!(j.xpath.contains("child/") && j.xpath.contains("toys/"));
    Ok(())
}

#[test]
fn checks_error_on_add_wrong_node() -> Result<()> {
    let actual = format!(
        "{}",
        RJini::empty()
            .add_node("so me no de")
            .unwrap_err()
            .root_cause()
    );
    assert!(actual.contains("The \"so me no de\" contain spaces"));
    Ok(())
}

#[test]
fn checks_removes_node() -> Result<()> {
    let j = RJini::empty()
        .add_node("Ruby")?
        .add_node("is")?
        .add_node("not")?
        .add_node("my")?
        .add_node("dog")?
        .remove_node("not");
    assert_eq!("Ruby/is/my/dog/", j.xpath);
    Ok(())
}

#[test]
fn checks_replaces_node() -> Result<()> {
    let j = RJini::empty()
        .add_node("Ruby")?
        .add_node("is")?
        .add_node("a")?
        .add_node("bad")?
        .add_node("dog")?
        .replace_node("bad", "good")?
        .xpath;
    assert_eq!("Ruby/is/a/good/dog/", j);
    Ok(())
}

#[test]
fn checks_error_on_replaces_node() -> Result<()> {
    let actual = format!(
        "{}",
        RJini::empty()
            .add_node("test")?
            .replace_node("test", "not test")
            .unwrap_err()
            .root_cause()
    );
    assert!(actual.contains("The \"not test\" contain spaces"));
    Ok(())
}
