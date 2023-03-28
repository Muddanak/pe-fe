use phf::phf_map;

#[allow(dead_code)]
pub static MAGIC: phf::Map<&'static str, u16> = phf_map!(
    "PE32" => 0x10b,
    "PE32+" => 0x20b,
    "ROM" => 0x107,
);

#[allow(dead_code)]
pub static SUBSYSTEM: phf::Map<&'static str, u16> = phf_map!(
    "NATIVE" =>	0x01,
    "WINDOWS_GUI " =>	0x02,
    "WINDOWS_CUI " =>	0x03,
    "OS2_CUI " =>	0x05,
    "POSIX_CUI " =>	0x07,
    "NATIVE_WINDOWS " =>	0x08,
    "WINDOWS_CE_GUI " =>	0x09,
    "EFI_APPLICATION " =>	0x0a,
    "EFI_BOOT_ SERVICE_DRIVER " =>	0x0b,
    "EFI_RUNTIME_ DRIVER " =>	0x0c,
    "EFI_ROM " =>	0x0d,
    "XBOX " =>	0x0e,
    "WINDOWS_BOOT_APPLICATION " =>	0x0f,
);

#[allow(dead_code)]
pub static DLL_CHARACTERISTICS: phf::Map<&'static str, u16> = phf_map!(
    "HIGH_ENTROPY_VA" =>	0x0020,
    "DYNAMIC_BASE" =>	0x0040,
    "FORCE_INTEGRITY" =>	0x0080,
    "NX_COMPAT" =>	0x0100,
    "NO_ISOLATION" =>	0x0200,
    "NO_SEH" =>	0x0400,
    "NO_BIND" =>	0x0800,
    "APPCONTAINER" =>	0x1000,
    "WDM_DRIVER" =>	0x2000,
    "GUARD_CF" =>	0x4000,
    "TERMINAL_SERVER_AWARE" =>	0x8000,
);
