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


/// from tokio
/// 1. 通过编译条件宏，只编译需要的代码
/// 2. spawn是一个多态函数，因为运行时只有一种可能，这里没有使用trait object来动态分派,直接使用枚举来手工分派
///    和场景有关，并且性能更好
/// 3. 可以使用类型参数（泛型）实现静态分派。
///
/// --- source
cfg_rt_core! {
    use crate::runtime::basic_scheduler;
    use crate::task::JoinHandle;

    use std::future::Future;
}

cfg_rt_threaded! {
    use crate::runtime::thread_pool;
}

#[derive(Debug, Clone)]
pub(crate) enum Spawner {
    Shell,
    #[cfg(feature = "rt-core")]
    Basic(basic_scheduler::Spawner),
    #[cfg(feature = "rt-threaded")]
    ThreadPool(thread_pool::Spawner),
}

cfg_rt_core! {
    impl Spawner {
        pub(crate) fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
        where
            F: Future + Send + 'static,
            F::Output: Send + 'static,
        {
            match self {
                Spawner::Shell => panic!("spawning not enabled for runtime"),
                #[cfg(feature = "rt-core")]
                Spawner::Basic(spawner) => spawner.spawn(future),
                #[cfg(feature = "rt-threaded")]
                Spawner::ThreadPool(spawner) => spawner.spawn(future),
            }
        }
    }
}
