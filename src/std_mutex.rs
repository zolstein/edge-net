use embassy_util::blocking_mutex::raw::RawMutex;

pub struct StdRawMutex(std::sync::Mutex<()>);

unsafe impl RawMutex for StdRawMutex {
    const INIT: Self = Self(std::sync::Mutex::new(()));

    fn lock<R>(&self, f: impl FnOnce() -> R) -> R {
        let _guard = self.0.lock().unwrap();

        f()
    }
}
