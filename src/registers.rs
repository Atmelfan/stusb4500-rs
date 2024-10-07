#![allow(non_upper_case_globals)]

use bitflags::bitflags;

pub enum Register {
    BcdTypeCRevL = 0x06,
    BcdTypeCRevH = 0x07,
    BcdUsbPDRevL = 0x08,
    BcdUsbPDRevH = 0x09,
    DeviceCapabHigh = 0x0A,
    AlertStatus1 = 0x0B,
    AlertStatus1Mask = 0x0C,
    PortStatus0 = 0x0D,
    PortStatus1 = 0x0E,
    TypeCMonitoringStatus0 = 0x0F,
    TypeCMonitoringStatus1 = 0x10,
    CCStatus = 0x11,
    CCHWFaultStatus0 = 0x12,
    CCHWFaultStatus1 = 0x13,
    PDTypeCStatus = 0x14,
    TypeCStatus = 0x15,
    PRTStatus = 0x16,
    //0x17-0x19, Reserved
    PDCommandCtrl = 0x1A,
    //0x1B-0x1F, Reserved
    MonitoringCtrl0 = 0x20,
    MonitoringCtrl1 = 0x21,
    MonitoringCtrl2 = 0x22,
    ResetCtrl = 0x23,
    //0x24, Reserved
    VbusDischargeTimeCtrl = 0x25,
    VbusDischargeCtrl = 0x26,
    VbusCtrl = 0x27,
    //0x28, Reserved
    PEFSM = 0x29,
    //0x2B, Reserved
    //0x2C, Reserved
    GpioSWGpio = 0x2D,
    //0x2E, Reserved
    DeviceId = 0x2F,
    //0x30, Reserved
    RXHeaderL = 0x31,
    RXHeaderH = 0x32,
    RXDataObj = 0x33, //4 bytes
    TXHeaderL = 0x51,
    TXHeaderH = 0x52,
    RWBuffer = 0x53, // 8 bytes
    //0x5B-0x6F, Reserved
    DPMPDONumb = 0x70,
    //0x71-0x84, Reserved
    DPMSNKPDO1 = 0x85,   //4 bytes
    DPMSNKPDO2 = 0x89,   //4 bytes
    DPMSNKPDO3 = 0x8D,   //4 bytes
    RDORegStatus = 0x91, //4 bytes
    NvmPassword = 0x95,
    NvmCtrl0 = 0x96,
    NvmCtrl1 = 0x97,
}

bitflags! {
    pub struct AlertMask: u8 {
        const PortStatus            = 0b0100_0000;
        const TypeCMonitoringStatus = 0b0010_0000;
        const CCFaultStatus         = 0b0001_0000;
        const PRTStatus             = 0b0000_0010;

        const _Default = Self::PortStatus.bits() | Self::TypeCMonitoringStatus.bits() | Self::PRTStatus.bits();
    }
}

impl Default for AlertMask {
    fn default() -> Self {
        AlertMask::_Default
    }
}

bitflags! {
    pub struct Alert: u8 {
        const PortStatus            = 0b0100_0000;
        const TypeCMonitoringStatus = 0b0010_0000;
        const CCHWFaultStatus       = 0b0001_0000;
        const PDTypeCStatus         = 0b0000_1000;
        const PRTStatus             = 0b0000_0010;
    }
}

bitflags! {
    pub struct NvmCtrl0: u8 {
        const Power   = 0b1000_0000;
        const Enable  = 0b0100_0000;
        const Request = 0b0001_0000;
    }
}

bitflags! {
    pub struct NvmCtrl1: u8 {
        const EraseSector0 = 0b0000_1000;
        const EraseSector1 = 0b0001_0000;
        const EraseSector2 = 0b0010_0000;
        const EraseSector3 = 0b0100_0000;
        const EraseSector4 = 0b1000_0000;
    }
}

pub enum NvmCtrl1Opcode {
    ReadSector = 0x00,   // Read the sector data
    LoadPlr = 0x01,      // Load the Program Load Register
    LoadSer = 0x02,      // Load the Sector Erase Register
    DumpPlr = 0x03,      // Dump the Program Load Register
    DumpSer = 0x04,      // Dump the Sector Erase Register
    EraseSectors = 0x05, // Erase the specified sectors
    WriteSector = 0x06,  // Program the sector data to EEPROM
}
