#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global OTG control and status register (USBFS_GOTGCS)"]
    pub gotgcs: GOTGCS,
    #[doc = "0x04 - Global OTG interrupt flag register (USBFS_GOTGINTF)"]
    pub gotgintf: GOTGINTF,
    #[doc = "0x08 - Global AHB control and status register (USBFS_GAHBCS)"]
    pub gahbcs: GAHBCS,
    #[doc = "0x0c - Global USB control and status register (USBFS_GUSBCSR)"]
    pub gusbcs: GUSBCS,
    #[doc = "0x10 - Global reset control register (USBFS_GRSTCTL)"]
    pub grstctl: GRSTCTL,
    #[doc = "0x14 - Global interrupt flag register (USBFS_GINTF)"]
    pub gintf: GINTF,
    #[doc = "0x18 - Global interrupt enable register (USBFS_GINTEN)"]
    pub ginten: GINTEN,
    #[doc = "0x1c - Global Receive status read(Device mode)"]
    pub grstatr_device: GRSTATR_DEVICE,
    #[doc = "0x20 - Global Receive status pop(Device mode)"]
    pub grstatp_device: GRSTATP_DEVICE,
    #[doc = "0x24 - Global Receive FIFO size register (USBFS_GRFLEN)"]
    pub grflen: GRFLEN,
    #[doc = "0x28 - Host non-periodic transmit FIFO length register (Host mode)"]
    pub hnptflen: HNPTFLEN,
    #[doc = "0x2c - Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
    pub hnptfqstat: HNPTFQSTAT,
    _reserved0: [u8; 8usize],
    #[doc = "0x38 - Global core configuration register (USBFS_GCCFG)"]
    pub gccfg: GCCFG,
    #[doc = "0x3c - core ID register"]
    pub cid: CID,
    _reserved1: [u8; 192usize],
    #[doc = "0x100 - Host periodic transmit FIFO length register (HPTFLEN)"]
    pub hptflen: HPTFLEN,
    #[doc = "0x104 - device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
    pub diep1tflen: DIEP1TFLEN,
    #[doc = "0x108 - device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
    pub diep2tflen: DIEP2TFLEN,
    #[doc = "0x10c - device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
    pub diep3tflen: DIEP3TFLEN,
}
#[doc = "Global OTG control and status register (USBFS_GOTGCS)"]
pub struct GOTGCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global OTG control and status register (USBFS_GOTGCS)"]
pub mod gotgcs;
#[doc = "Global OTG interrupt flag register (USBFS_GOTGINTF)"]
pub struct GOTGINTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global OTG interrupt flag register (USBFS_GOTGINTF)"]
pub mod gotgintf;
#[doc = "Global AHB control and status register (USBFS_GAHBCS)"]
pub struct GAHBCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global AHB control and status register (USBFS_GAHBCS)"]
pub mod gahbcs;
#[doc = "Global USB control and status register (USBFS_GUSBCSR)"]
pub struct GUSBCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global USB control and status register (USBFS_GUSBCSR)"]
pub mod gusbcs;
#[doc = "Global reset control register (USBFS_GRSTCTL)"]
pub struct GRSTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global reset control register (USBFS_GRSTCTL)"]
pub mod grstctl;
#[doc = "Global interrupt flag register (USBFS_GINTF)"]
pub struct GINTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global interrupt flag register (USBFS_GINTF)"]
pub mod gintf;
#[doc = "Global interrupt enable register (USBFS_GINTEN)"]
pub struct GINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global interrupt enable register (USBFS_GINTEN)"]
pub mod ginten;
#[doc = "Global Receive status read(Device mode)"]
pub struct GRSTATR_DEVICE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Receive status read(Device mode)"]
pub mod grstatr_device;
#[doc = "Global Receive status read(Host mode)"]
pub struct GRSTATR_HOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Receive status read(Host mode)"]
pub mod grstatr_host;
#[doc = "Global Receive status pop(Device mode)"]
pub struct GRSTATP_DEVICE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Receive status pop(Device mode)"]
pub mod grstatp_device;
#[doc = "Global Receive status pop(Host mode)"]
pub struct GRSTATP_HOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Receive status pop(Host mode)"]
pub mod grstatp_host;
#[doc = "Global Receive FIFO size register (USBFS_GRFLEN)"]
pub struct GRFLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Receive FIFO size register (USBFS_GRFLEN)"]
pub mod grflen;
#[doc = "Host non-periodic transmit FIFO length register (Host mode)"]
pub struct HNPTFLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host non-periodic transmit FIFO length register (Host mode)"]
pub mod hnptflen;
#[doc = "Device IN endpoint 0 transmit FIFO length (Device mode)"]
pub struct DIEP0TFLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device IN endpoint 0 transmit FIFO length (Device mode)"]
pub mod diep0tflen;
#[doc = "Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
pub struct HNPTFQSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
pub mod hnptfqstat;
#[doc = "Global core configuration register (USBFS_GCCFG)"]
pub struct GCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global core configuration register (USBFS_GCCFG)"]
pub mod gccfg;
#[doc = "core ID register"]
pub struct CID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "core ID register"]
pub mod cid;
#[doc = "Host periodic transmit FIFO length register (HPTFLEN)"]
pub struct HPTFLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host periodic transmit FIFO length register (HPTFLEN)"]
pub mod hptflen;
#[doc = "device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
pub struct DIEP1TFLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
pub mod diep1tflen;
#[doc = "device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
pub struct DIEP2TFLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
pub mod diep2tflen;
#[doc = "device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
pub struct DIEP3TFLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
pub mod diep3tflen;
