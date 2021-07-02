use std::collections::HashMap;
use crate::runtime::{Vm, Error as RuntimeError};
use crate::value::{Value, Veracity, Text, Book};
use crate::value::ops::{
	ConvertTo, Dump,
	Add, Subtract,
	IsEqual, Compare,
	GetIndex, SetIndex, GetAttr
};

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Codex(HashMap<Value, Value>);

impl Codex {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self(HashMap::with_capacity(capacity))
	}

	pub fn get(&self, key: &Value) -> Option<&Value> {
		self.0.get(key)
	}

	pub fn contains_key(&self, key: &Value) -> bool {
		self.0.contains_key(key)
	}

	pub fn get_mut(&mut self, key: &Value) -> Option<&mut Value> {
		self.0.get_mut(key)
	}

	pub fn insert(&mut self, key: Value, value: Value) -> Option<Value> {
		self.0.insert(key, value)
	}

	pub fn remove(&mut self, key: &Value) -> Option<Value> {
		self.0.remove(key)
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub fn iter(&self) -> impl Iterator<Item=(&Value, &Value)> {
		self.0.iter()
	}
}

impl From<HashMap<Value, Value>> for Codex {
	#[inline]
	fn from(hashmap: HashMap<Value, Value>) -> Self {
		Self(hashmap)
	}
}
impl IntoIterator for Codex {
	type Item = (Value, Value);
	type IntoIter = <HashMap<Value, Value> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()	
	}
}

impl std::iter::FromIterator<(Value, Value)> for Codex {
	fn from_iter<I: IntoIterator<Item=(Value, Value)>>(iter: I) -> Self {
		Self(iter.into_iter().collect())
	}
}

impl std::iter::Extend<(Value, Value)> for Codex {
	fn extend<I: IntoIterator<Item=(Value, Value)>>(&mut self, iter: I) {
		self.0.extend(iter)
	}
}

impl<I> std::ops::Index<&I> for Codex
where
	I: Eq + Hash,
	Value: Borrow<I>
{
	type Output = Value;

	#[inline]
	fn index(&self, index: &I) -> &Value {
		&self.0[index]
	}
}

impl<I: IntoIterator<Item=(Value, Value)>> std::ops::Add<I> for Codex {
	type Output = Self;

	fn add(mut self, rhs: I) -> Self::Output {
		self.0.extend(rhs);
		self
	}
}

impl<'a, I: IntoIterator<Item=&'a Value>> std::ops::Sub<I> for Codex {
	type Output = Self;

	fn sub(mut self, rhs: I) -> Self::Output {
		// todo: optimize
		let rhs = rhs.into_iter().collect::<Vec<_>>();
		self.0.retain(|value, _| !rhs.contains(&value));
		self
	}
}

impl Hash for Codex {
	fn hash<H: Hasher>(&self, h: &mut H) {
		0.hash(h); // todo: actually hash.
	}
}

impl From<Codex> for Value {
	#[inline]
	fn from(codex: Codex) -> Self {
		Self::Codex(codex)
	}
}

impl Dump for Codex {
	fn dump(&self, to: &mut String, vm: &mut Vm) -> Result<(), RuntimeError> {
		to.push('{');

		let mut is_first = true;

		for (key, value) in &self.0 {
			if is_first {
				is_first = false;
			} else {
				to.push_str(", ");
			}

			key.dump(to, vm)?;
			to.push_str(": ");
			value.dump(to, vm)?;
		}

		to.push('}');
		Ok(())
	}
}

impl ConvertTo<Veracity> for Codex {
	fn convert(&self, _: &mut Vm) -> Result<Veracity, RuntimeError> {
		Ok(!self.is_empty())
	}
}

impl ConvertTo<Text> for Codex {
	fn convert(&self, _: &mut Vm) -> Result<Text, RuntimeError> {
		todo!()
	}
}

impl ConvertTo<Book> for Codex {
	fn convert(&self, _: &mut Vm) -> Result<Book, RuntimeError> {
		todo!()
	}
}

impl Add for Codex {
	fn add(&self, rhs: &Value, vm: &mut Vm) -> Result<Value, RuntimeError> {
		let _ = (rhs, vm);
		todo!()
	}
}

impl Subtract for Codex {
	fn subtract(&self, rhs: &Value, vm: &mut Vm) -> Result<Value, RuntimeError> {
		let _ = (rhs, vm);
		todo!()
	}
}

impl IsEqual for Codex {
	fn is_equal(&self, rhs: &Value, vm: &mut Vm) -> Result<bool, RuntimeError> {
		let _ = (rhs, vm);
		todo!()
	}
}

impl Compare for Codex {
	fn compare(&self, rhs: &Value, vm: &mut Vm) -> Result<Option<Ordering>, RuntimeError> {
		let _ = (rhs, vm);
		todo!()
	}
}

impl GetIndex for Codex {
	fn get_index(&self, key: &Value, vm: &mut Vm) -> Result<Value, RuntimeError> {
		let _ = (key, vm);
		todo!()
	}
}

impl SetIndex for Codex {
	fn set_index(&self, key: Value, value: Value, vm: &mut Vm) -> Result<(), RuntimeError> {
		let _ = (key, value, vm);
		todo!()
	}
}

impl GetAttr for Codex {
	fn get_attr(&self, attr: &str, vm: &mut Vm) -> Result<Value, RuntimeError> {
		let _ = (attr, vm); todo!();
	}
}
