use aarch64_cpu::registers::Readable;
use aarch64_cpu::registers::CurrentEL;

pub fn get_el() -> Option<CurrentEL::EL::Value> {
    return CurrentEL.read_as_enum(CurrentEL::EL);
}
