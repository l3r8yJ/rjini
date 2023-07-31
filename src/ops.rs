use std::ops::Add;
use std::panic::Location;

use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;

use crate::RJini;

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
        Self::validate(node)?;
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
        Self::validate(new)?;
        let x = self.xpath.replace(origin, new);
        Ok(RJini { xpath: x })
    }

    /// It splits the xpath string into a vector of strings.
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let x = RJini::from("parent/child[@key=\"value\"]/next[3]");
    /// assert_eq!(
    ///     vec!["parent", "child[@key=\"value\"]", "next[3]"],
    ///     x.nodes().unwrap()
    /// )
    /// ```
    /// Returns:
    ///
    /// A vector of strings.
    pub fn nodes(&self) -> Result<Vec<&str>> {
        let regex = Regex::new(r"(//|/)")?;
        Ok(regex
            .split(&self.xpath)
            .filter(|node| !node.is_empty())
            .collect())
    }

    /// `add_property` adds a property to the current XPATH
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let j = RJini::from("parent/child/")
    ///     .add_property("p").unwrap();
    /// assert!(j.xpath.contains("p()"))
    /// ```
    /// Arguments:
    ///
    /// * `property`: The name of the property to add.
    ///
    /// Returns:
    ///
    /// A Result<RJini>
    pub fn add_property(&self, property: &str) -> Result<RJini> {
        Self::validate(&property)?;
        Ok(Self::add_node(
            self,
            &(String::from(property) + "()").as_str(),
        )?)
    }

    /// Removes a property from the current XPATH
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let j = RJini::from("parent/child/property()");
    /// assert!(
    ///     "parent/child/".eq(j.remove_property("property").xpath.as_str())
    /// );
    /// ```
    ///
    /// Arguments:
    ///
    /// * `property`: The property to remove.
    ///
    /// Returns:
    ///
    /// A new RJini object with the property removed from the xpath.
    pub fn remove_property(&self, property: &str) -> RJini {
        let x = self
            .xpath
            .clone()
            .replace((String::from(property) + "()").as_str(), "");
        RJini { xpath: x }
    }

    /// Adds attr to xpath.
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let mut j = RJini::from("parent/child")
    ///     .add_attr("k", "v")
    ///     .unwrap();
    /// assert_eq!(j.as_str(), "parent/child[k=\"v\"]");
    /// ```
    ///
    /// Arguments:
    ///
    /// * `key`: The attr key.
    ///
    /// * `value`: The attr value.
    ///
    /// Returns:
    ///
    /// A new RJini object with the new attr.
    pub fn add_attr(&self, key: &str, value: &str) -> Result<RJini> {
        let node = format!("[{}=\"{}\"]", key, value);
        Self::validate(node.as_str())?;
        let x = self.xpath.clone() + node.as_str();
        Ok(RJini { xpath: x })
    }

    /// Represents RJini as string.
    ///
    /// For example:
    /// ```
    /// use rjini::RJini;
    /// let j = RJini::from("parent/child/property()");
    /// assert_eq!(j.as_str(), "parent/child/property()");
    /// ```
    /// Returns:
    ///
    /// A rjini's xpath represented as string.
    pub fn as_str(&self) -> &str {
        self.xpath.as_str()
    }

    /// It checks if the node contains spaces.
    ///
    /// Arguments:
    ///
    /// * `node`: The name of the node to validate.
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
}

#[test]
fn checks_creates_rjini_from() -> Result<()> {
    let j = RJini::from("parent/child");
    assert!(j.xpath.contains("child"));
    Ok(())
}

#[test]
fn creates_rjini_from_complex_xpath() -> Result<()> {
    let rj = RJini::from("/bookstore/book[price>35]/price");
    assert_eq!(vec!["bookstore", "book[price>35]", "price"], rj.nodes()?);
    Ok(())
}

#[test]
fn checks_adds_node() -> Result<()> {
    let j = RJini::from("parent/");
    let j = j.add_node("child")?;
    let j = j.add_node("toys")?;
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

#[test]
fn checks_does_nodes() -> Result<()> {
    let x = RJini::from("parent/child[@key=\"value\"]/next[3]");
    assert_eq!(
        vec!["parent", "child[@key=\"value\"]", "next[3]"],
        x.nodes()?
    );
    Ok(())
}

#[test]
fn checks_adds_property() -> Result<()> {
    let x = RJini::from("some/xpath/");
    assert!(x.add_property("pr")?.xpath.contains("pr()"));
    Ok(())
}

#[test]
fn checks_removes_property() -> Result<()> {
    let x = RJini::from("some/xpath/");
    assert!(x.add_property("pr")?.xpath.contains("pr()"));
    let x = x.remove_property("pr").xpath;
    assert!(!x.contains("pr"));
    Ok(())
}

#[test]
fn adds_new_attr() -> Result<()> {
    let rj = RJini::from("parent/child").add_attr("k", "v")?;
    assert_eq!(rj.as_str(), "parent/child[k=\"v\"]");
    Ok(())
}
