

use persy::{Config, PRes, Persy};
use std::sync::{Condvar, Mutex};
use tempfile::Builder;

#[allow(dead_code)]
pub struct CountDown {
    lock: Mutex<u64>,
    cond: Condvar,
}

#[allow(dead_code)]
impl CountDown {
    pub fn new(count: u64) -> CountDown {
        CountDown {
            lock: Mutex::new(count),
            cond: Condvar::new(),
        }
    }

    pub fn wait(&self) -> PRes<bool> {
        let guard = self.lock.lock()?;
        if *guard != 0 {
            let _ = self.cond.wait(guard)?;
        }
        Ok(true)
    }

    pub fn count_down(&self) -> PRes<()> {
        let mut count = self.lock.lock()?;
        *count = (*count) - 1;
        if *count == 0 {
            self.cond.notify_all();
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub fn create_and_drop<F>(name: &str, test: F)
where
    F: FnOnce(&Persy),
{
    create_and_drop_with_config(name, Config::new(), test);
}

#[allow(dead_code)]
pub fn create_and_drop_with_config<F>(name: &str, config: Config, test: F)
where
    F: FnOnce(&Persy),
{
    let file = Builder::new()
        .prefix(name)
        .suffix(".persy")
        .tempfile()
        .expect("expect temp file creation");
    Persy::create_from_file(file.reopen().expect("reopen")).expect(&format!("file '{:?}' do not exist", file));
    {
        let persy = Persy::open_from_file(file.reopen().expect("reopen"), config).unwrap();
        test(&persy);
    }
}
