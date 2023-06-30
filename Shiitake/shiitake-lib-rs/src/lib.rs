pub mod server;

use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::{jboolean, jdouble, jint};
use parking_lot::Mutex;
use serde::Serialize;

use sysinfo::{NetworkExt, System, SystemExt, CpuExt};

use single_value_channel::{Updater as SingleUpdater, channel_starting_with as single_channel, Receiver as SingleReceiver};


static STATS: Mutex<Option<SingleReceiver<MeasuredStats>>> = Mutex::new(None);

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize)]
struct MeasuredStats {
    cpu_usage: f64,
    cpu_freq: f64,
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
#[cfg(feature = "jni")]
pub extern "system" fn Java_Shiitake_init_1measurements<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jboolean {
    jbool(init_measurements())
}

pub fn init_measurements() -> bool {
    if STATS.lock().is_some() {
        return false;
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
            stats.cpu_freq = system.global_cpu_info().frequency() as f64;

            updater.update(stats).ok();

            let elapsed = start.elapsed();
            let sleep_time = std::time::Duration::from_millis(1000) - elapsed;
            std::thread::sleep(sleep_time);
        }
    });


    STATS.lock().replace(receiver);

    return true;
}

/// Returns the latest CPU usage as a percentage
#[no_mangle]
#[cfg(feature = "jni")]
pub extern "system" fn Java_Shiitake_cpu_1usage<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jdouble {
    cpu_usage()
}

pub fn cpu_usage() -> f64 {
    STATS.lock().as_mut().map(|stats| {
        stats.latest().cpu_usage
    }).unwrap_or(0.0)
}

/// Returns the latest CPU frequency in Hz
#[no_mangle]
#[cfg(feature = "jni")]
pub extern "system" fn Java_Shiitake_cpu_1frequency<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jdouble {
    cpu_frequency()
}

pub fn cpu_frequency() -> f64 {
    STATS.lock().as_mut().map(|stats| {
        stats.latest().cpu_freq
    }).unwrap_or(0.0)
}

/// Returns the amount of memory used by the system in bytes
#[no_mangle]
#[cfg(feature = "jni")]
pub extern "system" fn Java_Shiitake_memory_1usage<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jdouble {
    memory_usage()
}

pub fn memory_usage() -> f64 {
    STATS.lock().as_mut().map(|stats| {
        stats.latest().memory_usage
    }).unwrap_or(0.0)
}

/// Returns the amount of network data received by the system in bytes
#[no_mangle]
#[cfg(feature = "jni")]
pub extern "system" fn Java_Shiitake_network_1usage_1in<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jdouble {
    network_usage_in()
}

pub fn network_usage_in() -> f64 {
    STATS.lock().as_mut().map(|stats| {
        stats.latest().network_usage_in
    }).unwrap_or(0.0)
}

/// Returns the amount of network data transmitted by the system in bytes
#[no_mangle]
#[cfg(feature = "jni")]
pub extern "system" fn Java_Shiitake_network_1usage_1out<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jdouble {
    network_usage_out()
}

pub fn network_usage_out() -> f64 {
    STATS.lock().as_mut().map(|stats| {
        stats.latest().network_usage_out
    }).unwrap_or(0.0)
}

/// Retursn the os summary
#[no_mangle]
#[cfg(feature = "jni")]
pub extern "system" fn Java_Shiitake_os_1version<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> JString<'local> {
    env.new_string(os_version()).expect("Couldn't create java string")
}

pub fn os_version() -> String {
    let mut sys = System::new();
    sys.refresh_all();
    let name = sys.name().unwrap_or("Unknown".to_string());
    let os_version = sys.os_version().unwrap_or("Unknown".to_string());
    let os_kernel = sys.kernel_version().unwrap_or("Unknown".to_string());
    let os = format!("name: {}, version: {}, kernel: {}", name, os_version, os_kernel);
    os
}

/// Returns cpu cores
#[no_mangle]
#[cfg(feature = "jni")]
pub extern "system" fn Java_Shiitake_cpu_1cores<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jint {
    cpu_cores()
}

pub fn cpu_cores() -> i32 {
    let mut sys = System::new();
    sys.refresh_all();
    let cores = sys.cpus().len() as i32;
    cores
}

/// Returns the total amount of memory in bytes
#[no_mangle]
#[cfg(feature = "jni")]
pub extern "system" fn Java_Shiitake_memory_1total<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jdouble {
    memory_total()
}

pub fn memory_total() -> f64 {
    let mut sys = System::new();
    sys.refresh_all();
    let total = sys.total_memory() as f64;
    total
}
