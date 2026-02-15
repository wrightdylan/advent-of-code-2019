#[macro_export]
macro_rules! hashset {
    // hashset!() returns a new empty hashset
    () => {
        ::std::collections::HashSet::new()
    };
    
    // hashset!(element; count) returns a HashSet containing count copies of element
    ($elem:expr; $n:expr) => {
        {
            let mut set = ::std::collections::HashSet::new();
            for _ in 0..$n {
                set.insert($elem);
            }
            set
        }
    };
    
    // hashset!(elem1, elem2, elem3, ...) returns a HashSet containing all the specified elements.
    ($($x:expr),+ $(,)?) => {
        {
            let mut set = ::std::collections::HashSet::new();
            $(
                set.insert($x);
            )+
            set
        }
    };
}

#[macro_export]
macro_rules! hashmap {
    // hashmap!() returns a new empty hashmap
    () => {
        ::std::collections::HashMap::new()
    };

    // hashmap!(key1 => value1, key2 => value2, key3 => value3, ...) returns a HashMap containing all of the specified paird
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! vecdeque {
    ($($elem:expr),*) => {{
        let mut dq = ::std::collections::VecDeque::new();
        $(
            dq.push_back($elem);
        )*
        dq
    }};
}