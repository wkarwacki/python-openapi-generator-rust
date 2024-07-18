pub(crate) fn ensure<F>(f: F)
where
    F: Fn() -> bool,
{
    if !f() {
        todo!("Implement validation")
    }
}
