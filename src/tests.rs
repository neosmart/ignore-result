use crate::Ignore;

#[test]
fn basic_ignore() -> Result<(), ()> {
    fn important_func() -> Result<(), ()> {
        Ok(())
    }

    fn unimportant_func() -> Result<(), ()> {
        Ok(())
    }

    let foo = important_func();
    if foo.is_err() {
        // Run some cleanup or output some diagnostics But the important
        // return value is `foo`, and it must be preserved, i.e. any
        // failure in this block should not take precedence over the error
        // contained in `foo`
        unimportant_func().ignore();
        return foo;
    }

    Ok(())
}
