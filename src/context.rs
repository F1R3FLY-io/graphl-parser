pub(crate) const INNER_PLACEHOLDER: &str = "%inner";

pub(crate) fn get_context(context: *mut String) -> Option<&'static mut String> {
    if context.is_null() || context.is_null() {
        None
    } else {
        unsafe { Some(&mut *context) }
    }
}

pub(crate) fn save_context(context: *mut String, content: String) {
    unsafe {
        if let Some(ctx) = context.as_mut() {
            *ctx = (*ctx).replace(INNER_PLACEHOLDER, &content);
        }
    };
}

#[cfg(test)]
pub mod test {
    use std::str::FromStr;

    use crate::context::{INNER_PLACEHOLDER, get_context, save_context};

    #[test]
    fn test_get_context() {
        let context = Box::new(String::from_str("Hello, world").unwrap());
        let ptr = Box::into_raw(context);

        let result = get_context(ptr).unwrap().clone();

        assert_eq!(result, String::from_str("Hello, world").unwrap());
    }

    #[test]
    fn test_save_context() {
        let context = Box::new(String::from(format!("Hello, world! {}", INNER_PLACEHOLDER)));
        let ptr = Box::into_raw(context);

        save_context(ptr, "Good bey, world!".into());

        // reconstruct context from pointer
        let context = unsafe { Box::from_raw(ptr) };

        assert_eq!(
            *context,
            String::from_str("Hello, world! Good bey, world!").unwrap()
        );
    }
}
