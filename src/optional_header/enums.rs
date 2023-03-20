use phf::phf_map;

#[allow(dead_code)]
pub static MAGIC: phf::Map<&'static str, u16> = phf_map!(
    "PE32" => 0x10b,
    "PE32+" => 0x20b,
);

#[allow(dead_code)]
pub static SUBSYSTEM: phf::Map<&'static str, u16> = phf_map!(
    "UNKNOWN " =>	0x0000,
    "NATIVE" =>	1,
    "WINDOWS_GUI " =>	2,
    "WINDOWS_CUI " =>	3,
    "OS2_CUI " =>	5,
    "POSIX_CUI " =>	7,
    "NATIVE_WINDOWS " =>	8,
    "WINDOWS_CE_GUI " =>	9,
    "EFI_APPLICATION " =>	10,
    "EFI_BOOT_ SERVICE_DRIVER " =>	11,
    "EFI_RUNTIME_ DRIVER " =>	12,
    "EFI_ROM " =>	13,
    "XBOX " =>	14,
    "WINDOWS_BOOT_APPLICATION " =>	16,
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
