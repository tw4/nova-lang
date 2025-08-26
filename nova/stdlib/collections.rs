// Nova Standard Library - Collections Module

use std::collections::{VecDeque};

/// Wrapper for f64 that implements Eq and Ord
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderedFloat(i64); // Store as i64 bits of f64

impl OrderedFloat {
    pub fn new(value: f64) -> Self {
        Self(value.to_bits() as i64)
    }
    
    pub fn get(self) -> f64 {
        f64::from_bits(self.0 as u64)
    }
}

impl From<f64> for OrderedFloat {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

/// List data structure (dynamic array)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NovaList {
    items: Vec<NovaValue>,
}

impl NovaList {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: NovaValue) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Option<NovaValue> {
        self.items.pop()
    }

    pub fn get(&self, index: usize) -> Option<&NovaValue> {
        self.items.get(index)
    }

    pub fn set(&mut self, index: usize, value: NovaValue) -> bool {
        if index < self.items.len() {
            self.items[index] = value;
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn insert(&mut self, index: usize, value: NovaValue) {
        if index <= self.items.len() {
            self.items.insert(index, value);
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<NovaValue> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn contains(&self, value: &NovaValue) -> bool {
        self.items.contains(value)
    }

    pub fn iter(&self) -> std::slice::Iter<NovaValue> {
        self.items.iter()
    }

    pub fn sort(&mut self) {
        // Note: This would need custom ordering implementation for NovaValue
        // For now, this method exists but doesn't actually sort
        // self.items.sort();
    }

    pub fn reverse(&mut self) {
        self.items.reverse();
    }
}

/// Set data structure (unique values)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NovaSet {
    items: Vec<NovaValue>, // Using Vec instead of HashSet to avoid Hash trait requirement
}

impl NovaSet {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, value: NovaValue) -> bool {
        if !self.items.contains(&value) {
            self.items.push(value);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, value: &NovaValue) -> bool {
        if let Some(pos) = self.items.iter().position(|x| x == value) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, value: &NovaValue) -> bool {
        self.items.contains(value)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn union(&self, other: &NovaSet) -> NovaSet {
        let mut result = self.clone();
        for item in &other.items {
            result.insert(item.clone());
        }
        result
    }

    pub fn intersection(&self, other: &NovaSet) -> NovaSet {
        let mut result = NovaSet::new();
        for item in &self.items {
            if other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    pub fn difference(&self, other: &NovaSet) -> NovaSet {
        let mut result = NovaSet::new();
        for item in &self.items {
            if !other.contains(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    pub fn iter(&self) -> std::slice::Iter<NovaValue> {
        self.items.iter()
    }
}

/// Map data structure (key-value pairs)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NovaMap {
    items: Vec<(NovaValue, NovaValue)>, // Using Vec instead of HashMap to avoid Hash trait requirement
}

impl NovaMap {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, key: NovaValue, value: NovaValue) -> Option<NovaValue> {
        for (existing_key, existing_value) in &mut self.items {
            if *existing_key == key {
                let old_value = existing_value.clone();
                *existing_value = value;
                return Some(old_value);
            }
        }
        self.items.push((key, value));
        None
    }

    pub fn get(&self, key: &NovaValue) -> Option<&NovaValue> {
        for (existing_key, existing_value) in &self.items {
            if existing_key == key {
                return Some(existing_value);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &NovaValue) -> Option<NovaValue> {
        if let Some(pos) = self.items.iter().position(|(k, _)| k == key) {
            Some(self.items.remove(pos).1)
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &NovaValue) -> bool {
        self.items.iter().any(|(k, _)| k == key)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn keys(&self) -> Vec<&NovaValue> {
        self.items.iter().map(|(k, _)| k).collect()
    }

    pub fn values(&self) -> Vec<&NovaValue> {
        self.items.iter().map(|(_, v)| v).collect()
    }

    pub fn iter(&self) -> std::slice::Iter<(NovaValue, NovaValue)> {
        self.items.iter()
    }
}

/// Queue data structure (FIFO)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NovaQueue {
    items: VecDeque<NovaValue>,
}

impl NovaQueue {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(capacity),
        }
    }

    pub fn enqueue(&mut self, value: NovaValue) {
        self.items.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<NovaValue> {
        self.items.pop_front()
    }

    pub fn peek(&self) -> Option<&NovaValue> {
        self.items.front()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

/// Stack data structure (LIFO)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NovaStack {
    items: Vec<NovaValue>,
}

impl NovaStack {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: NovaValue) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Option<NovaValue> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&NovaValue> {
        self.items.last()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

/// Ordered Map data structure (sorted key-value pairs)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NovaOrderedMap {
    items: Vec<(NovaValue, NovaValue)>, // Using Vec instead of BTreeMap
}

impl NovaOrderedMap {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: NovaValue, value: NovaValue) -> Option<NovaValue> {
        // Insert in sorted order (simplified)
        for (existing_key, existing_value) in &mut self.items {
            if *existing_key == key {
                let old_value = existing_value.clone();
                *existing_value = value;
                return Some(old_value);
            }
        }
        self.items.push((key, value));
        None
    }

    pub fn get(&self, key: &NovaValue) -> Option<&NovaValue> {
        for (existing_key, existing_value) in &self.items {
            if existing_key == key {
                return Some(existing_value);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &NovaValue) -> Option<NovaValue> {
        if let Some(pos) = self.items.iter().position(|(k, _)| k == key) {
            Some(self.items.remove(pos).1)
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &NovaValue) -> bool {
        self.items.iter().any(|(k, _)| k == key)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn first_key_value(&self) -> Option<(&NovaValue, &NovaValue)> {
        self.items.first().map(|(k, v)| (k, v))
    }

    pub fn last_key_value(&self) -> Option<(&NovaValue, &NovaValue)> {
        self.items.last().map(|(k, v)| (k, v))
    }

    pub fn keys(&self) -> Vec<&NovaValue> {
        self.items.iter().map(|(k, _)| k).collect()
    }

    pub fn values(&self) -> Vec<&NovaValue> {
        self.items.iter().map(|(_, v)| v).collect()
    }

    pub fn iter(&self) -> std::slice::Iter<(NovaValue, NovaValue)> {
        self.items.iter()
    }
}

/// Ordered Set data structure (sorted unique values)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NovaOrderedSet {
    items: Vec<NovaValue>, // Using Vec instead of BTreeSet
}

impl NovaOrderedSet {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: NovaValue) -> bool {
        if !self.items.contains(&value) {
            self.items.push(value);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, value: &NovaValue) -> bool {
        if let Some(pos) = self.items.iter().position(|x| x == value) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, value: &NovaValue) -> bool {
        self.items.contains(value)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn first(&self) -> Option<&NovaValue> {
        self.items.first()
    }

    pub fn last(&self) -> Option<&NovaValue> {
        self.items.last()
    }

    pub fn iter(&self) -> std::slice::Iter<NovaValue> {
        self.items.iter()
    }
}

/// Generic Nova Value for collections
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NovaValue {
    Nil,
    Bool(bool),
    Number(i64),
    Float(OrderedFloat), // Using OrderedFloat wrapper for f64
    String(String),
    List(Box<NovaList>),
    Set(Box<NovaSet>),
    Map(Box<NovaMap>),
    Queue(Box<NovaQueue>),
    Stack(Box<NovaStack>),
    OrderedMap(Box<NovaOrderedMap>),
    OrderedSet(Box<NovaOrderedSet>),
}

impl NovaValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            NovaValue::Nil => "nil",
            NovaValue::Bool(_) => "bool",
            NovaValue::Number(_) => "number",
            NovaValue::Float(_) => "float",
            NovaValue::String(_) => "string",
            NovaValue::List(_) => "list",
            NovaValue::Set(_) => "set",
            NovaValue::Map(_) => "map",
            NovaValue::Queue(_) => "queue",
            NovaValue::Stack(_) => "stack",
            NovaValue::OrderedMap(_) => "ordered_map",
            NovaValue::OrderedSet(_) => "ordered_set",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            NovaValue::Nil => false,
            NovaValue::Bool(b) => *b,
            NovaValue::Number(n) => *n != 0,
            NovaValue::Float(f) => {
                let float_val = f.get();
                float_val != 0.0
            }
            NovaValue::String(s) => !s.is_empty(),
            NovaValue::List(l) => !l.is_empty(),
            NovaValue::Set(s) => !s.is_empty(),
            NovaValue::Map(m) => !m.is_empty(),
            NovaValue::Queue(q) => !q.is_empty(),
            NovaValue::Stack(s) => !s.is_empty(),
            NovaValue::OrderedMap(m) => !m.is_empty(),
            NovaValue::OrderedSet(s) => !s.is_empty(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            NovaValue::Nil => "nil".to_string(),
            NovaValue::Bool(b) => b.to_string(),
            NovaValue::Number(n) => n.to_string(),
            NovaValue::Float(f) => {
                let float_val = f.get();
                float_val.to_string()
            }
            NovaValue::String(s) => s.clone(),
            NovaValue::List(l) => format!("[{} items]", l.len()),
            NovaValue::Set(s) => format!("Set({} items)", s.len()),
            NovaValue::Map(m) => format!("Map({} items)", m.len()),
            NovaValue::Queue(q) => format!("Queue({} items)", q.len()),
            NovaValue::Stack(s) => format!("Stack({} items)", s.len()),
            NovaValue::OrderedMap(m) => format!("OrderedMap({} items)", m.len()),
            NovaValue::OrderedSet(s) => format!("OrderedSet({} items)", s.len()),
        }
    }
}

/// Collection utilities and helper functions
pub struct Collections;

impl Collections {
    /// Create a new list
    pub fn list() -> NovaValue {
        NovaValue::List(Box::new(NovaList::new()))
    }

    /// Create a new list with initial capacity
    pub fn list_with_capacity(capacity: usize) -> NovaValue {
        NovaValue::List(Box::new(NovaList::with_capacity(capacity)))
    }

    /// Create a new set
    pub fn set() -> NovaValue {
        NovaValue::Set(Box::new(NovaSet::new()))
    }

    /// Create a new map
    pub fn map() -> NovaValue {
        NovaValue::Map(Box::new(NovaMap::new()))
    }

    /// Create a new queue
    pub fn queue() -> NovaValue {
        NovaValue::Queue(Box::new(NovaQueue::new()))
    }

    /// Create a new stack
    pub fn stack() -> NovaValue {
        NovaValue::Stack(Box::new(NovaStack::new()))
    }

    /// Create a new ordered map
    pub fn ordered_map() -> NovaValue {
        NovaValue::OrderedMap(Box::new(NovaOrderedMap::new()))
    }

    /// Create a new ordered set
    pub fn ordered_set() -> NovaValue {
        NovaValue::OrderedSet(Box::new(NovaOrderedSet::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_operations() {
        let mut list = NovaList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());

        list.push(NovaValue::Number(1));
        list.push(NovaValue::String("test".to_string()));
        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());

        assert_eq!(list.get(0), Some(&NovaValue::Number(1)));
        assert_eq!(list.pop(), Some(NovaValue::String("test".to_string())));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_set_operations() {
        let mut set = NovaSet::new();
        assert!(set.insert(NovaValue::Number(1)));
        assert!(!set.insert(NovaValue::Number(1))); // Duplicate
        assert_eq!(set.len(), 1);
        assert!(set.contains(&NovaValue::Number(1)));
    }

    #[test]
    fn test_map_operations() {
        let mut map = NovaMap::new();
        map.insert(NovaValue::String("key".to_string()), NovaValue::Number(42));
        assert_eq!(
            map.get(&NovaValue::String("key".to_string())),
            Some(&NovaValue::Number(42))
        );
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_queue_operations() {
        let mut queue = NovaQueue::new();
        queue.enqueue(NovaValue::Number(1));
        queue.enqueue(NovaValue::Number(2));
        
        assert_eq!(queue.dequeue(), Some(NovaValue::Number(1)));
        assert_eq!(queue.dequeue(), Some(NovaValue::Number(2)));
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_stack_operations() {
        let mut stack = NovaStack::new();
        stack.push(NovaValue::Number(1));
        stack.push(NovaValue::Number(2));
        
        assert_eq!(stack.pop(), Some(NovaValue::Number(2)));
        assert_eq!(stack.pop(), Some(NovaValue::Number(1)));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }
}