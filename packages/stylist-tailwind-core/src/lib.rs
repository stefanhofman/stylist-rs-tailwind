pub mod components;

macro_rules! combine_classes {
    ( $( $x:expr ),* ) => {
        {
            let mut map: HashMap<&'static str, &'static str> = HashMap::new();
            $(
                for (key, value) in $x.into_iter() {
                    map.insert(key, value);
                }
            )*
            map
        }
    }
}

pub(crate) use combine_classes;
