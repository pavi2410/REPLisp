use crate::{
    error::{Error, ReplispResult},
    eval::register_builtins,
    lval::{add, qexpr, sym, Lval},
};
use std::{collections::HashMap, fmt};

pub type LenvLookupTable = HashMap<String, Box<Lval>>;

#[derive(Debug)]
pub struct Lenv<'a> {
    lookup: LenvLookupTable,
    parent: Option<&'a Lenv<'a>>,
}

impl<'a> Lenv<'a> {
    /// Create a new Lenv with an optional parent.
    ///
    /// * `lookup` - The lookup table to use for this Lenv
    /// * `parent` - The parent Lenv
    pub fn new(lookup: Option<LenvLookupTable>, parent: Option<&'a Lenv<'a>>) -> Self {
        let mut ret = Self {
            lookup: lookup.unwrap_or_default(),
            parent,
        };

        register_builtins(&mut ret);

        ret
    }

    /// Get a value from the Lenv. If the value is not found, check the parent.
    ///
    /// * `key` - The key to lookup
    pub fn get(&self, key: &str) -> ReplispResult<Box<Lval>> {
        match self.lookup.get(key) {
            Some(value) => Ok(value.clone()),
            None => match self.parent {
                Some(parent) => parent.get(key),
                None => Err(Error::UnknownFunction(key.to_string())),
            },
        }
    }

    /// Returns an Lval containing Symbols with each k,v pair in the local env
    pub fn list_all(&self) -> ReplispResult<Box<Lval>> {
        let mut ret = qexpr();
        for (k, v) in &self.lookup {
            add(&mut ret, &sym(&format!("{k}:{v}")))?;
        }
        Ok(ret)
    }

    /// Put a value into the Lenv.
    ///
    /// * `key` - The key to use
    /// * `value` - The value to store
    pub fn put(&mut self, key: String, value: Box<Lval>) {
        self.lookup.insert(key, value);
    }
}

impl<'a> fmt::Display for Lenv<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let parent_str = if self.parent.is_some() {
			"Child"
		} else {
			"Root"
		};
		write!(f, "{} vals in env | {}", self.lookup.len(), parent_str)
	}
}