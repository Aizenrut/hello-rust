use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, K, V>
where
    T: FnMut(K) -> V,
{
    calculation: T,
    map: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: FnMut(K) -> V,
    K: Eq + Hash + Copy,
    V: Copy,
{
    pub fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: K) -> V {
        match self.map.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.map.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn with_integers() {
        let mut count = 0;
        let mut cached_closure = Cacher::new(|x| {
            count += 1;
            x
        });

        assert_eq!(1, cached_closure.value(1));
        assert_eq!(2, cached_closure.value(2));
        assert_eq!(1, cached_closure.value(1));
        assert_eq!(2, cached_closure.value(2));
        assert_eq!(2, count);
    }

    #[test]
    fn with_strings() {
        let mut count = 0;
        let mut cached_closure = Cacher::new(|x| {
            count += 1;
            x
        });

        assert_eq!("abc", cached_closure.value("abc"));
        assert_eq!("xyz", cached_closure.value("xyz"));
        assert_eq!("abc", cached_closure.value("abc"));
        assert_eq!("xyz", cached_closure.value("xyz"));
        assert_eq!(2, count);
    }

    #[test]
    fn with_bools() {
        let mut count = 0;
        let mut cached_closure = Cacher::new(|x| {
            count += 1;
            x
        });

        assert_eq!(true, cached_closure.value(true));
        assert_eq!(false, cached_closure.value(false));
        assert_eq!(true, cached_closure.value(true));
        assert_eq!(false, cached_closure.value(false));
        assert_eq!(2, count);
    }
}
