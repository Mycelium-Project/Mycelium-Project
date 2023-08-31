use num_enum::{IntoPrimitive, TryFromPrimitive};

const IP_SUFFIX : &str = "/nisysapi/server";


#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum DeviceTags {
    Serial = 16805888,
    UserName = 16904192,
    Model = 16801792,
    Firmware = 16969728,
    Os = 17104896,
    ImageVersion = 219529216
}

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum ResourceTags {
    Temp = 16965632, // not being received
    CpuLoad1 = 17141760,
    CpuLoad2 = 17141761,
    CpuSpeed1 = 17309696,
    CpuSpeed2 = 17309697,
    RamTotal = 219480064,
    RamFree = 219484160,
    VirtRamTotal = 219492352,
    VirtRamFree = 219496448,
    DiskTotal = 219291648,
    DiskFree = 219295744,
}

const RsrcTags: [ResourceTags; 11] = [
    ResourceTags::Temp,
    ResourceTags::CpuLoad1,
    ResourceTags::CpuLoad2,
    ResourceTags::CpuSpeed1,
    ResourceTags::CpuSpeed2,
    ResourceTags::RamTotal,
    ResourceTags::RamFree,
    ResourceTags::VirtRamTotal,
    ResourceTags::VirtRamFree,
    ResourceTags::DiskTotal,
    ResourceTags::DiskFree,
];

fn u32_to_hex_string(val: u32) -> String {
    format!("{:08x}", val)
}
fn hex_string_to_u32(val: &str) -> u32 {
    u32::from_str_radix(val, 16).unwrap()
}