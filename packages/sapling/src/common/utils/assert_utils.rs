pub fn assert_value(value: bool) -> Result<(), ()> {
    if value {
        Ok(())
    } else {
        Err(())
    }
}

pub fn assert_value_or_error<E>(value: bool, error: E) -> Result<(), E> {
    assert_value(value).map_err(|_| error)
}

pub fn assert_predicate<P>(predicate: P) -> Result<(), ()>
    where P: Fn() -> bool {
    
    assert_value(predicate())
}

pub fn assert_predicate_or_error<P, E>(predicate: P, error: E) -> Result<(), E> 
    where P: Fn() -> bool {

    assert_predicate(predicate).map_err(|_| error)
}