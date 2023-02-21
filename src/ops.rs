use crate::RJini;
use anyhow::anyhow;
use anyhow::Result;

/// Creating a new instance of RJini from a XPATH as string.
impl RJini {
    /// It takes a string and returns a RJini object.
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let j = RJini::from("parent/child".to_string());
    /// assert!(j.body.contains("t/c"))
    /// ```
    ///
    /// Arguments:
    ///
    /// * `xpath`: The XPath expression to be evaluated.
    ///
    /// Returns:
    ///
    /// A struct with a body field.
    pub fn from(xpath: String) -> Self {
        RJini { body: xpath }
    }

    /// It adds a node to the body of the XPATH.
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let j = RJini::from("parent/".to_string())
    ///     .add_node("child".to_string()).unwrap()
    ///     .add_node("game".to_string()).unwrap();
    /// assert!(j.body.contains("child/game/"))
    /// ```
    ///
    /// Arguments:
    ///
    /// * `node`: The node to add to the XPATH.
    ///
    /// Returns:
    ///
    /// A new RJini object with the new body.
    pub fn add_node(&self, node: String) -> Result<RJini> {
        if node.contains(" ") {
            return Err(anyhow!(format!("#add_node: The {node} contain spaces")));
        }
        let b = self.body.clone() + &node + "/";
        Ok(RJini { body: b })
    }
}

#[test]
fn checks_creates_rjini_from() -> Result<()> {
    let j = RJini::from("parent/child".to_string());
    assert!(j.body.contains("child"));
    Ok(())
}

#[test]
fn checks_adds_a_node() -> Result<()> {
    let j = RJini::from("parent/".to_string());
    let j = j.add_node("child".to_string())?;
    let j = j.add_node("toys".to_string())?;
    println!("{}", j.body);
    assert!(j.body.contains("child/") && j.body.contains("toys/"));
    Ok(())
}
