pub type Cstring = *const u8;
pub type Wcstring = *const char;
pub type Ibool = u32;
pub type Data = u32;
pub type Address = u32;
pub type Cycle = u32;

#[derive(Debug)]
pub enum NesError {
    /**
     * NTSC/PAL region mismatch.
     */
    WRONG_MODE,
    /**
     * Missing FDS BIOS.
     */
    MISSING_BIOS,
    /**
     * Unsupported or malformed mapper.
     */
    UNSUPPORTED_MAPPER,
    /**
     * Vs DualSystem is unsupported.
     */
    UNSUPPORTED_VSSYSTEM,
    /**
     * File format version is no longer supported.
     */
    UNSUPPORTED_FILE_VERSION,
    /**
     * Unsupported operation.
     */
    UNSUPPORTED,
    /**
     * Invalid CRC checksum.
     */
    INVALID_CRC,
    /**
     * Corrupt file.
     */
    CORRUPT_FILE,
    /**
     * Invalid file.
     */
    INVALID_FILE,
    /**
     * Invalid parameter(s).
     */
    INVALID_PARAM,
    /**
     * System not ready.
     */
    NOT_READY,
    /**
     * Out of memory.
     */
    OUT_OF_MEMORY,
    /**
     * Generic error.
     */
    GENERIC,
}

#[derive(Debug)]
pub enum NesWarning {
    /**
     * Success.
     */
    OK,
    /**
     * Success but operation had no effect.
     */
    NOP,
    /**
     * Success but image dump may be bad.
     */
    BAD_DUMP,
    /**
     * Success but PRG-ROM may be bad.
     */
    BAD_PROM,
    /**
     * Success but CHR-ROM may be bad.
     */
    BAD_CROM,
    /**
     * Success but file header may have incorrect data.
     */
    BAD_FILE_HEADER,
    /**
     * Success but save data has been lost.
     */
    SAVEDATA_LOST,
    /**
     * Success but data may have been replaced.
     */
    DATA_REPLACED,
}

#[derive(Debug)]
pub enum Region {
    NTSC,
    PAL,
}

#[derive(Debug)]
pub enum System {
    NTSC,
    PAL,
    PAL_A,
    PAL_B,
    FAMICOM,
    DENDY,
    VS_UNISYSTEM,
    VS_DUALSYSTEM,
    PLAYCHOICE_10,
}

#[derive(Debug)]
pub enum FavoredSystem {
    NTSC,
    PAL,
    FAMICOM,
    DENDY,
}

#[derive(Debug)]
pub enum CpuModel {
    RP2A03,
    RP2A07,
    DENDY,
}

#[derive(Debug)]
pub enum PpuModel {
    RP2C02,
    RP2C03B,
    RP2C03G,
    RP2C04_0001,
    RP2C04_0002,
    RP2C04_0003,
    RP2C04_0004,
    RC2C03B,
    RC2C03C,
    RC2C05_01,
    RC2C05_02,
    RC2C05_03,
    RC2C05_04,
    RC2C05_05,
    RP2C07,
    DENDY,
}

#[derive(Debug)]
pub enum Mirroring {
    HORIZONTAL,
    VERTICAL,
    FOURSCREEN,
}

pub const CLK_M2_MUL: u32 = 6;
pub const CLK_NTSC: u32 = 39375000 * CLK_M2_MUL;
pub const CLK_NTSC_DIV: u32 = 11;
pub const CLK_NTSC_HVSYNC: u32 = 525 * 455 * CLK_NTSC_DIV * CLK_M2_MUL / 4;
pub const CLK_PAL: u32 = 35468950 * CLK_M2_MUL;
pub const CLK_PAL_DIV: u32 = 8;
pub const CLK_PAL_HVSYNC: u32 = 625 * 1418758 / (10000 / CLK_PAL_DIV) * CLK_M2_MUL;

pub const CPU_RP2A03_CC: u32 = 12;
pub const CPU_RP2A07_CC: u32 = 16;
pub const CPU_DENDY_CC: u32 = 15;

pub const PPU_RP2C02_CC: u32 = 4;
pub const PPU_RP2C02_HACTIVE: u32 = PPU_RP2C02_CC * 256;
pub const PPU_RP2C02_HBLANK: u32 = PPU_RP2C02_CC * 85;
pub const PPU_RP2C02_HSYNC: u32 = PPU_RP2C02_HACTIVE + PPU_RP2C02_HBLANK;
pub const PPU_RP2C02_VACTIVE: u32 = 240;
pub const PPU_RP2C02_VSLEEP: u32 = 1;
pub const PPU_RP2C02_VINT: u32 = 20;
pub const PPU_RP2C02_VDUMMY: u32 = 1;
pub const PPU_RP2C02_VBLANK: u32 = PPU_RP2C02_VSLEEP + PPU_RP2C02_VINT + PPU_RP2C02_VDUMMY;
pub const PPU_RP2C02_VSYNC: u32 = PPU_RP2C02_VACTIVE + PPU_RP2C02_VBLANK;
pub const PPU_RP2C02_HVSYNCBOOT: u32 = PPU_RP2C02_VACTIVE * PPU_RP2C02_HSYNC + PPU_RP2C02_CC * 312;
pub const PPU_RP2C02_HVREGBOOT: u32 =
    (PPU_RP2C02_VACTIVE + PPU_RP2C02_VINT) * PPU_RP2C02_HSYNC + PPU_RP2C02_CC * 314;
pub const PPU_RP2C02_HVINT: u32 = PPU_RP2C02_VINT * PPU_RP2C02_HSYNC;
pub const PPU_RP2C02_HVSYNC_0: u32 = PPU_RP2C02_VSYNC * PPU_RP2C02_HSYNC;
pub const PPU_RP2C02_HVSYNC_1: u32 = PPU_RP2C02_VSYNC * PPU_RP2C02_HSYNC - PPU_RP2C02_CC;
pub const PPU_RP2C02_HVSYNC: u32 = (PPU_RP2C02_HVSYNC_0 + PPU_RP2C02_HVSYNC_1) / 2;
pub const PPU_RP2C02_FPS: u32 =
    (CLK_NTSC + CLK_NTSC_DIV * PPU_RP2C02_HVSYNC / 2) / (CLK_NTSC_DIV * PPU_RP2C02_HVSYNC);
pub const PPU_RP2C07_CC: u32 = 5;
pub const PPU_RP2C07_HACTIVE: u32 = PPU_RP2C07_CC * 256;
pub const PPU_RP2C07_HBLANK: u32 = PPU_RP2C07_CC * 85;
pub const PPU_RP2C07_HSYNC: u32 = PPU_RP2C07_HACTIVE + PPU_RP2C07_HBLANK;
pub const PPU_RP2C07_VACTIVE: u32 = 240;
pub const PPU_RP2C07_VSLEEP: u32 = 1;
pub const PPU_RP2C07_VINT: u32 = 70;
pub const PPU_RP2C07_VDUMMY: u32 = 1;
pub const PPU_RP2C07_VBLANK: u32 = PPU_RP2C07_VSLEEP + PPU_RP2C07_VINT + PPU_RP2C07_VDUMMY;
pub const PPU_RP2C07_VSYNC: u32 = PPU_RP2C07_VACTIVE + PPU_RP2C07_VBLANK;
pub const PPU_RP2C07_HVSYNCBOOT: u32 = PPU_RP2C07_VACTIVE * PPU_RP2C07_HSYNC + PPU_RP2C07_CC * 312;
pub const PPU_RP2C07_HVREGBOOT: u32 =
    (PPU_RP2C07_VACTIVE + PPU_RP2C07_VINT) * PPU_RP2C07_HSYNC + PPU_RP2C07_CC * 314;
pub const PPU_RP2C07_HVINT: u32 = PPU_RP2C07_VINT * PPU_RP2C07_HSYNC;
pub const PPU_RP2C07_HVSYNC: u32 = PPU_RP2C07_VSYNC * PPU_RP2C07_HSYNC;
pub const PPU_RP2C07_FPS: u32 =
    (CLK_PAL + CLK_PAL_DIV * PPU_RP2C07_HVSYNC / 2) / (CLK_PAL_DIV * PPU_RP2C07_HVSYNC);
pub const PPU_DENDY_CC: u32 = 5;
pub const PPU_DENDY_HACTIVE: u32 = PPU_DENDY_CC * 256;
pub const PPU_DENDY_HBLANK: u32 = PPU_DENDY_CC * 85;
pub const PPU_DENDY_HSYNC: u32 = PPU_DENDY_HACTIVE + PPU_DENDY_HBLANK;
pub const PPU_DENDY_VACTIVE: u32 = 240;
pub const PPU_DENDY_VSLEEP: u32 = 51;
pub const PPU_DENDY_VINT: u32 = 20;
pub const PPU_DENDY_VDUMMY: u32 = 1;
pub const PPU_DENDY_VBLANK: u32 = PPU_DENDY_VSLEEP + PPU_DENDY_VINT + PPU_DENDY_VDUMMY;
pub const PPU_DENDY_VSYNC: u32 = PPU_DENDY_VACTIVE + PPU_DENDY_VBLANK;
pub const PPU_DENDY_HVSYNCBOOT: u32 = PPU_DENDY_VACTIVE * PPU_DENDY_HSYNC + PPU_DENDY_CC * 312;
pub const PPU_DENDY_HVREGBOOT: u32 =
    (PPU_DENDY_VACTIVE + PPU_DENDY_VINT) * PPU_DENDY_HSYNC + PPU_DENDY_CC * 314;
pub const PPU_DENDY_HVINT: u32 = PPU_DENDY_VINT * PPU_DENDY_HSYNC;
pub const PPU_DENDY_HVSYNC: u32 = PPU_DENDY_VSYNC * PPU_DENDY_HSYNC;
pub const PPU_DENDY_FPS: u32 =
    (CLK_PAL + CLK_PAL_DIV * PPU_DENDY_HVSYNC / 2) / (CLK_PAL_DIV * PPU_DENDY_HVSYNC);

pub const SIZE_1K: usize = 0x400;
pub const SIZE_2K: usize = 0x800;
pub const SIZE_4K: usize = 0x1000;
pub const SIZE_5K: usize = 0x1400;
pub const SIZE_6K: usize = 0x1800;
pub const SIZE_8K: usize = 0x2000;
pub const SIZE_16K: usize = 0x4000;
pub const SIZE_32K: usize = 0x8000;
pub const SIZE_40K: usize = 0xA000;
pub const SIZE_64K: usize = 0x10000;
pub const SIZE_128K: usize = 0x20000;
pub const SIZE_256K: usize = 0x40000;
pub const SIZE_512K: usize = 0x80000;
pub const SIZE_1024K: usize = 0x100000;
pub const SIZE_2048K: usize = 0x200000;
pub const SIZE_3072K: usize = 0x300000;
pub const SIZE_4096K: usize = 0x400000;
pub const SIZE_8192K: usize = 0x800000;
pub const SIZE_16384K: usize = 0x1000000;

pub const NMI_VECTOR: u16 = 0xFFFA;
pub const RESET_VECTOR: u16 = 0xFFFC;
pub const IRQ_VECTOR: u16 = 0xFFFE;
pub const RESET_CYCLES: u32 = 7;
pub const INT_CYCLES: u32 = 7;
pub const BRK_CYCLES: u32 = 7;
pub const RTI_CYCLES: u32 = 6;
pub const RTS_CYCLES: u32 = 6;
pub const PHA_CYCLES: u32 = 3;
pub const PHP_CYCLES: u32 = 3;
pub const PLA_CYCLES: u32 = 4;
pub const PLP_CYCLES: u32 = 4;
pub const JSR_CYCLES: u32 = 6;
pub const JMP_ABS_CYCLES: u32 = 3;
pub const JMP_IND_CYCLES: u32 = 5;
