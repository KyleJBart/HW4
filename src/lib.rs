use std::collections::HashMap;

/// Returns the first `n` Fibonacci numbers.
pub fn fib(n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    
    if n == 0 {
        return result;
    }
    
    result.push(0);
    if n == 1 {
        return result;
    }
    
    result.push(1);
    if n == 2 {
        return result;
    }
    
    for i in 2..n as usize {
        let next = result[i-1] + result[i-2];
        result.push(next);
    }
    
    result
}

/// Returns true if `n` is a palindrome, false otherwise.
pub fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

/// Returns the nth largest element in `a`, or None if it does not exist.
pub fn nthmax(n: usize, a: &[i32]) -> Option<i32> {
    if n >= a.len() {
        return None;
    }
    
    let mut sorted = a.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    
    Some(sorted[n])
}

/// Returns a one-character String containing the most frequent character in `s`.
pub fn freq(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    
    let mut char_count = HashMap::new();
    
    for c in s.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }
    
    char_count
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(c, _)| c.to_string())
        .unwrap_or_default()
}

/// Zips two slices into a HashMap, mapping arr1[i] -> arr2[i].
pub fn zip_hash(
    arr1: &[String],
    arr2: &[String],
) -> Option<HashMap<String, String>> {
    if arr1.len() != arr2.len() {
        return None;
    }
    
    let mut map = HashMap::new();
    
    for i in 0..arr1.len() {
        map.insert(arr1[i].clone(), arr2[i].clone());
    }
    
    Some(map)
}

/// Converts a HashMap into a Vec of (key, value) pairs.
pub fn hash_to_array(
    map: &HashMap<String, String>,
) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = map
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

// ========================
// Part 2: PhoneBook
// ========================

/// A single phone book entry.
#[derive(Debug, Clone)]
pub struct PhoneEntry {
    pub name: String,
    pub number: String,
    pub is_listed: bool,
}

/// PhoneBook holds name/number pairs and whether each is listed.
#[derive(Debug, Default)]
pub struct PhoneBook {
    // You are free to change this internal representation if you want.
    pub entries: Vec<PhoneEntry>,
}

impl PhoneBook {
    /// Constructor: create an empty PhoneBook.
    pub fn new() -> Self {
        // You may also use `Self::default()`
        PhoneBook {
            entries: Vec::new(),
        }
    }

    /// Attempts to add a new entry.
    ///
    /// Rules:
    /// 1. If the name already exists, return false.
    /// 2. If the number is not in the format NNN-NNN-NNNN, return false.
    /// 3. A number can be unlisted any number of times, but listed at most once.
    ///    - If the number already exists as listed, adding another listed entry
    ///      with the same number must return false.
    ///
    /// Returns true if the entry was successfully added.
    pub fn add(
        &mut self,
        name: String,
        number: String,
        is_listed: bool,
    ) -> bool {
        // Rule 1: Check if name already exists
        if self.entries.iter().any(|entry| entry.name == name) {
            return false;
        }
        
        // Rule 2: Validate phone number format
        if !PhoneBook::is_valid_number(&number) {
            return false;
        }
        
        // Rule 3: Check listed number restrictions
        if is_listed {
            // Check if this number is already listed
            if self.entries.iter().any(|entry| entry.number == number && entry.is_listed) {
                return false;
            }
        }
        
        // All checks passed, add the entry
        self.entries.push(PhoneEntry {
            name,
            number,
            is_listed,
        });
        
        true
    }
    
    /// Helper function to validate phone number format
    fn is_valid_number(number: &str) -> bool {
        let parts: Vec<&str> = number.split('-').collect();
        
        // Check format: NNN-NNN-NNNN
        if parts.len() != 3 {
            return false;
        }
        
        // Check each part length
        if parts[0].len() != 3 || parts[1].len() != 3 || parts[2].len() != 4 {
            return false;
        }
        
        // Check that all characters are digits
        parts.iter().all(|part| part.chars().all(|c| c.is_ascii_digit()))
    }

    /// Looks up `name` and returns the number ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup(&self, name: &str) -> Option<String> {
        self.entries
            .iter()
            .find(|entry| entry.name == name && entry.is_listed)
            .map(|entry| entry.number.clone())
    }

    /// Looks up `num` and returns the associated name ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup_by_num(&self, num: &str) -> Option<String> {
        self.entries
            .iter()
            .find(|entry| entry.number == num && entry.is_listed)
            .map(|entry| entry.name.clone())
    }

    /// Returns all names (listed and unlisted) whose numbers begin with `areacode`.
    pub fn names_by_ac(&self, areacode: &str) -> Vec<String> {
        let mut result = Vec::new();
        
        for entry in &self.entries {
            // Check if number starts with area code followed by '-'
            if entry.number.starts_with(&format!("{}-", areacode)) {
                result.push(entry.name.clone());
            }
        }
        
        result
    }
}