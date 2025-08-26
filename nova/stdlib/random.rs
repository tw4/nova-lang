// Nova Standard Library - Random Module

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

/// Linear Congruential Generator for random number generation
#[derive(Debug, Clone)]
pub struct NovaRng {
    seed: u64,
}

impl NovaRng {
    /// Create a new RNG with a given seed
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Create a new RNG with a time-based seed
    pub fn new_seeded() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        
        let mut hasher = DefaultHasher::new();
        now.hash(&mut hasher);
        
        Self {
            seed: hasher.finish(),
        }
    }

    /// Generate next random u64
    pub fn next_u64(&mut self) -> u64 {
        // Linear congruential generator parameters (same as used in glibc)
        const A: u64 = 1103515245;
        const C: u64 = 12345;
        const M: u64 = 1 << 31;

        self.seed = (A.wrapping_mul(self.seed).wrapping_add(C)) % M;
        self.seed
    }

    /// Generate random u32
    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// Generate random float between 0.0 and 1.0
    pub fn next_f64(&mut self) -> f64 {
        let max = (1u64 << 53) as f64;
        (self.next_u64() >> 11) as f64 / max
    }

    /// Generate random float between 0.0 and 1.0
    pub fn next_f32(&mut self) -> f32 {
        self.next_f64() as f32
    }

    /// Generate random integer in range [min, max)
    pub fn range_i64(&mut self, min: i64, max: i64) -> i64 {
        if min >= max {
            return min;
        }
        let range = (max - min) as u64;
        min + (self.next_u64() % range) as i64
    }

    /// Generate random integer in range [min, max)
    pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
        self.range_i64(min as i64, max as i64) as i32
    }

    /// Generate random float in range [min, max)
    pub fn range_f64(&mut self, min: f64, max: f64) -> f64 {
        if min >= max {
            return min;
        }
        min + (max - min) * self.next_f64()
    }

    /// Generate random float in range [min, max)
    pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        self.range_f64(min as f64, max as f64) as f32
    }

    /// Generate random boolean
    pub fn bool(&mut self) -> bool {
        self.next_u64() % 2 == 0
    }

    /// Generate random boolean with given probability of being true
    pub fn bool_with_probability(&mut self, probability: f64) -> bool {
        self.next_f64() < probability.clamp(0.0, 1.0)
    }

    /// Choose random element from slice
    pub fn choose<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            None
        } else {
            let index = self.range_i64(0, items.len() as i64) as usize;
            items.get(index)
        }
    }

    /// Shuffle a mutable slice using Fisher-Yates algorithm
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.range_i64(0, (i + 1) as i64) as usize;
            items.swap(i, j);
        }
    }

    /// Generate random bytes
    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(self.next_u64() as u8);
        }
        result
    }

    /// Generate random string of given length using alphanumeric characters
    pub fn alphanumeric_string(&mut self, len: usize) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        (0..len)
            .map(|_| {
                let idx = self.range_i64(0, CHARS.len() as i64) as usize;
                CHARS[idx] as char
            })
            .collect()
    }

    /// Generate random string of given length using custom character set
    pub fn string_from_chars(&mut self, len: usize, chars: &str) -> String {
        let char_bytes = chars.as_bytes();
        if char_bytes.is_empty() {
            return String::new();
        }

        (0..len)
            .map(|_| {
                let idx = self.range_i64(0, char_bytes.len() as i64) as usize;
                char_bytes[idx] as char
            })
            .collect()
    }

    /// Generate random UUID-like string
    pub fn uuid(&mut self) -> String {
        format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.next_u32(),
            self.next_u32() as u16,
            self.next_u32() as u16,
            self.next_u32() as u16,
            self.next_u64() & 0xffffffffffff
        )
    }

    /// Sample n items without replacement from a slice
    pub fn sample<T: Clone>(&mut self, items: &[T], n: usize) -> Vec<T> {
        if n >= items.len() {
            return items.to_vec();
        }

        let mut indices: Vec<usize> = (0..items.len()).collect();
        self.shuffle(&mut indices);
        
        indices.iter()
            .take(n)
            .map(|&i| items[i].clone())
            .collect()
    }

    /// Generate normally distributed random number (Box-Muller transform)
    pub fn normal(&mut self, mean: f64, std_dev: f64) -> f64 {
        // Box-Muller transform
        static mut SPARE: Option<f64> = None;
        static mut HAS_SPARE: bool = false;

        unsafe {
            if HAS_SPARE {
                HAS_SPARE = false;
                return SPARE.unwrap() * std_dev + mean;
            }

            HAS_SPARE = true;
            
            let u = self.next_f64();
            let v = self.next_f64();
            
            let mag = std_dev * (-2.0 * u.ln()).sqrt();
            let z0 = mag * (2.0 * std::f64::consts::PI * v).cos() + mean;
            let z1 = mag * (2.0 * std::f64::consts::PI * v).sin();
            
            SPARE = Some(z1);
            z0
        }
    }

    /// Generate exponentially distributed random number
    pub fn exponential(&mut self, lambda: f64) -> f64 {
        if lambda <= 0.0 {
            return 0.0;
        }
        -self.next_f64().ln() / lambda
    }

    /// Generate random number from uniform distribution
    pub fn uniform(&mut self, min: f64, max: f64) -> f64 {
        self.range_f64(min, max)
    }
}

/// Global random number generator instance
static mut GLOBAL_RNG: Option<NovaRng> = None;
static mut RNG_INITIALIZED: bool = false;

fn get_global_rng() -> &'static mut NovaRng {
    unsafe {
        if !RNG_INITIALIZED {
            GLOBAL_RNG = Some(NovaRng::new_seeded());
            RNG_INITIALIZED = true;
        }
        GLOBAL_RNG.as_mut().unwrap()
    }
}

/// Random utility functions using global RNG
pub struct Random;

impl Random {
    /// Seed the global random number generator
    pub fn seed(seed: u64) {
        unsafe {
            GLOBAL_RNG = Some(NovaRng::new(seed));
            RNG_INITIALIZED = true;
        }
    }

    /// Generate random integer in range [min, max)
    pub fn int(min: i64, max: i64) -> i64 {
        get_global_rng().range_i64(min, max)
    }

    /// Generate random float in range [0.0, 1.0)
    pub fn float() -> f64 {
        get_global_rng().next_f64()
    }

    /// Generate random float in range [min, max)
    pub fn float_range(min: f64, max: f64) -> f64 {
        get_global_rng().range_f64(min, max)
    }

    /// Generate random boolean
    pub fn bool() -> bool {
        get_global_rng().bool()
    }

    /// Generate random boolean with probability
    pub fn bool_with_prob(probability: f64) -> bool {
        get_global_rng().bool_with_probability(probability)
    }

    /// Choose random element from slice
    pub fn choice<T>(items: &[T]) -> Option<&T> {
        get_global_rng().choose(items)
    }

    /// Shuffle a mutable slice
    pub fn shuffle<T>(items: &mut [T]) {
        get_global_rng().shuffle(items)
    }

    /// Generate random bytes
    pub fn bytes(len: usize) -> Vec<u8> {
        get_global_rng().bytes(len)
    }

    /// Generate random alphanumeric string
    pub fn string(len: usize) -> String {
        get_global_rng().alphanumeric_string(len)
    }

    /// Generate random string with custom characters
    pub fn string_with_chars(len: usize, chars: &str) -> String {
        get_global_rng().string_from_chars(len, chars)
    }

    /// Generate random UUID
    pub fn uuid() -> String {
        get_global_rng().uuid()
    }

    /// Sample items without replacement
    pub fn sample<T: Clone>(items: &[T], n: usize) -> Vec<T> {
        get_global_rng().sample(items, n)
    }

    /// Generate normally distributed number
    pub fn normal(mean: f64, std_dev: f64) -> f64 {
        get_global_rng().normal(mean, std_dev)
    }

    /// Generate exponentially distributed number
    pub fn exponential(lambda: f64) -> f64 {
        get_global_rng().exponential(lambda)
    }

    /// Generate uniformly distributed number
    pub fn uniform(min: f64, max: f64) -> f64 {
        get_global_rng().uniform(min, max)
    }

    /// Roll a dice with given number of sides
    pub fn dice(sides: i64) -> i64 {
        Self::int(1, sides + 1)
    }

    /// Flip a coin (true = heads, false = tails)
    pub fn coin() -> bool {
        Self::bool()
    }

    /// Generate random color as hex string
    pub fn color() -> String {
        format!("#{:06x}", Self::int(0, 0x1000000))
    }

    /// Generate random color as RGB values
    pub fn rgb() -> (u8, u8, u8) {
        let rng = get_global_rng();
        (
            rng.range_i64(0, 256) as u8,
            rng.range_i64(0, 256) as u8,
            rng.range_i64(0, 256) as u8,
        )
    }

    /// Generate random HSL color values
    pub fn hsl() -> (f64, f64, f64) {
        let rng = get_global_rng();
        (
            rng.range_f64(0.0, 360.0), // Hue: 0-360
            rng.range_f64(0.0, 100.0), // Saturation: 0-100
            rng.range_f64(0.0, 100.0), // Lightness: 0-100
        )
    }
}

/// Weighted random selection
#[derive(Debug, Clone)]
pub struct WeightedChoice<T> {
    items: Vec<(T, f64)>,
    total_weight: f64,
}

impl<T: Clone> WeightedChoice<T> {
    /// Create new weighted choice
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            total_weight: 0.0,
        }
    }

    /// Add item with weight
    pub fn add(&mut self, item: T, weight: f64) {
        if weight > 0.0 {
            self.items.push((item, weight));
            self.total_weight += weight;
        }
    }

    /// Choose random item based on weights
    pub fn choose(&self, rng: &mut NovaRng) -> Option<&T> {
        if self.items.is_empty() || self.total_weight <= 0.0 {
            return None;
        }

        let mut random = rng.range_f64(0.0, self.total_weight);
        
        for (item, weight) in &self.items {
            if random < *weight {
                return Some(item);
            }
            random -= weight;
        }

        // Fallback to last item (shouldn't happen with proper weights)
        self.items.last().map(|(item, _)| item)
    }

    /// Choose random item using global RNG
    pub fn choose_global(&self) -> Option<&T> {
        self.choose(get_global_rng())
    }

    /// Get number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.total_weight = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_deterministic() {
        let mut rng1 = NovaRng::new(12345);
        let mut rng2 = NovaRng::new(12345);
        
        // Same seed should produce same sequence
        assert_eq!(rng1.next_u64(), rng2.next_u64());
        assert_eq!(rng1.next_u64(), rng2.next_u64());
    }

    #[test]
    fn test_rng_range() {
        let mut rng = NovaRng::new(12345);
        
        for _ in 0..100 {
            let val = rng.range_i64(10, 20);
            assert!(val >= 10 && val < 20);
        }
    }

    #[test]
    fn test_rng_float() {
        let mut rng = NovaRng::new(12345);
        
        for _ in 0..100 {
            let val = rng.next_f64();
            assert!(val >= 0.0 && val < 1.0);
        }
    }

    #[test]
    fn test_rng_choice() {
        let mut rng = NovaRng::new(12345);
        let items = [1, 2, 3, 4, 5];
        
        for _ in 0..10 {
            let choice = rng.choose(&items);
            assert!(choice.is_some());
            assert!(items.contains(choice.unwrap()));
        }
    }

    #[test]
    fn test_rng_shuffle() {
        let mut rng = NovaRng::new(12345);
        let mut items = [1, 2, 3, 4, 5];
        let original = items.clone();
        
        rng.shuffle(&mut items);
        
        // Should contain same elements
        items.sort();
        assert_eq!(items, original);
    }

    #[test]
    fn test_weighted_choice() {
        let mut choice = WeightedChoice::new();
        choice.add("a", 1.0);
        choice.add("b", 2.0);
        choice.add("c", 3.0);
        
        assert_eq!(choice.len(), 3);
        
        let mut rng = NovaRng::new(12345);
        let result = choice.choose(&mut rng);
        assert!(result.is_some());
        assert!(["a", "b", "c"].contains(result.unwrap()));
    }

    #[test]
    fn test_uuid_format() {
        let mut rng = NovaRng::new(12345);
        let uuid = rng.uuid();
        
        // UUID should be in format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().nth(8).unwrap(), '-');
        assert_eq!(uuid.chars().nth(13).unwrap(), '-');
        assert_eq!(uuid.chars().nth(18).unwrap(), '-');
        assert_eq!(uuid.chars().nth(23).unwrap(), '-');
    }

    #[test]
    fn test_global_random() {
        let int1 = Random::int(1, 10);
        let int2 = Random::int(1, 10);
        assert!(int1 >= 1 && int1 < 10);
        assert!(int2 >= 1 && int2 < 10);
        
        let float_val = Random::float();
        assert!(float_val >= 0.0 && float_val < 1.0);
    }
}