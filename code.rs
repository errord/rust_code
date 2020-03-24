/// from tokio
/// DropGuard 用来做些清理工作。可以用来模拟try finally.
///
/// --- source
///
/// Set this [`ThreadContext`] as the current active [`ThreadContext`].
///
/// [`ThreadContext`]: struct.ThreadContext.html
pub(crate) fn enter<F, R>(new: Handle, f: F) -> R
where
    F: FnOnce() -> R,
{
    struct DropGuard(Option<Handle>);

    impl Drop for DropGuard {
        fn drop(&mut self) {
            CONTEXT.with(|ctx| {
                *ctx.borrow_mut() = self.0.take();
            });
        }
    }

    let _guard = CONTEXT.with(|ctx| {
        let old = ctx.borrow_mut().replace(new);
        DropGuard(old)
    });

    f()
}
