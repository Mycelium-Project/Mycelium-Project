use std::cell::RefCell;

use jni::JNIEnv;

use jni::objects::{JClass};

use jni::sys::{jboolean, jdouble};

use sysinfo::{NetworkExt, System, SystemExt, CpuExt};

use single_value_channel::{Updater as SingleUpdater, channel_starting_with as single_channel, Receiver as SingleReceiver};

thread_local! {
    static STATS: RefCell<Option<SingleReceiver<MeasuredStats>>> = RefCell::new(None);
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct MeasuredStats {
    cpu_usage: f64,
    memory_usage: f64,
    network_usage_out: f64,
    network_usage_in: f64,
    supply_voltage: f64,
    supply_current: f64,
}

fn jbool(b: bool) -> jboolean {
    if b {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_Shiitake_init_measurements<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jboolean {
    if STATS.with(|stats| stats.borrow().is_some()) {
        return jbool(false);
    }

    let (receiver, sender) = single_channel(MeasuredStats::default());
    //spawn a new thread to repeatedly monitor system stats and update the channel
    std::thread::spawn(move || {
        let updater: SingleUpdater<MeasuredStats> = sender;

        let mut system = System::new_all();

        loop {
            let start = std::time::Instant::now();

            let mut stats = MeasuredStats::default();

            system.refresh_all();

            stats.cpu_usage = system.global_cpu_info().cpu_usage() as f64;
            stats.memory_usage = system.used_memory() as f64;
            for (_, data) in system.networks() {
                stats.network_usage_in += data.received() as f64;
                stats.network_usage_out += data.transmitted() as f64;
            }

            updater.update(stats).ok();

            let elapsed = start.elapsed();
            let sleep_time = std::time::Duration::from_millis(1000) - elapsed;
            std::thread::sleep(sleep_time);
        }
    });


    STATS.with(move |stats| {
        stats.replace(Some(receiver));
    });

    return jbool(true);
}

/// Returns the latest CPU usage as a percentage
#[no_mangle]
pub extern "system" fn Java_Shiitake_cpu_usage<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jdouble {
    STATS.with(|stats| {
        stats.borrow_mut().as_mut().map(|stats| {
            stats.latest().cpu_usage
        }).unwrap_or(0.0)
    })
}

/// Returns the amount of memory used by the system in bytes
#[no_mangle]
pub extern "system" fn Java_Shiitake_memory_usage<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jdouble {
    STATS.with(|stats| {
        stats.borrow_mut().as_mut().map(|stats| {
            stats.latest().memory_usage
        }).unwrap_or(0.0)
    })
}

/// Returns the amount of network data received by the system in bytes
#[no_mangle]
pub extern "system" fn Java_Shiitake_network_usage_in<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jdouble {
    STATS.with(|stats| {
        stats.borrow_mut().as_mut().map(|stats| {
            stats.latest().network_usage_in
        }).unwrap_or(0.0)
    })
}

/// Returns the amount of network data transmitted by the system in bytes
#[no_mangle]
pub extern "system" fn Java_Shiitake_network_usage_out<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jdouble {
    STATS.with(|stats| {
        stats.borrow_mut().as_mut().map(|stats| {
            stats.latest().network_usage_out
        }).unwrap_or(0.0)
    })
}
