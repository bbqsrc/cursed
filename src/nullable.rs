use crate::sync::Arc;

#[repr(transparent)]
pub struct Nullable<T>(*const T);

impl<T> Nullable<T> {
    pub fn new(ptr: *const T) -> Nullable<T> {
        Nullable(ptr)
    }
}

pub fn null<T>() -> Nullable<T> {
    Nullable(std::ptr::null())
}

impl<T> From<Arc<T>> for Nullable<Arc<T>> {
    fn from(thing: Arc<T>) -> Nullable<Arc<T>> {
        Nullable(&thing)
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(option: Option<T>) -> Nullable<T> {
        match option {
            Some(value) => Nullable(&value),
            None => null(),
        }
    }
}

// impl<T> From<Option<std::sync::Arc<T>>> for Nullable<T> {
//     fn from(option: Option<std::sync::Arc<T>>) -> Nullable<T> {
//         match option {
//             Some(value) => Nullable(std::sync::Arc::into_raw(value)),
//             None => null()
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStruct {
        field1: u64,
        field2: String,
    }

    #[test]
    fn is_not_null() {
        let thing = Some(TestStruct {
            field1: 523,
            field2: "oh no".into(),
        });
        let nullable = Nullable::from(thing);
        assert_ne!(nullable.0, null::<TestStruct>().0);
    }

    #[test]
    fn is_null() {
        let thing = None;
        let nullable = Nullable::from(thing);
        assert_eq!(nullable.0, null::<TestStruct>().0);
    }
}
