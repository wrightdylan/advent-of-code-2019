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
macro_rules! vecdeque {
    ($($elem:expr),*) => {{
        let mut dq = ::std::collections::VecDeque::new();
        $(
            dq.push_back($elem);
        )*
        dq
    }};
}