#[doc = r"Register block"]
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
    _reserved_7_grstatr: [u8; 0x04],
    _reserved_8_grstatp: [u8; 0x04],
    #[doc = "0x24 - Global Receive FIFO size register (USBFS_GRFLEN)"]
    pub grflen: GRFLEN,
    _reserved_10_hnptflen: [u8; 0x04],
    #[doc = "0x2c - Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
    pub hnptfqstat: HNPTFQSTAT,
    _reserved12: [u8; 0x08],
    #[doc = "0x38 - Global core configuration register (USBFS_GCCFG)"]
    pub gccfg: GCCFG,
    #[doc = "0x3c - core ID register"]
    pub cid: CID,
    _reserved14: [u8; 0xc0],
    #[doc = "0x100 - Host periodic transmit FIFO length register (HPTFLEN)"]
    pub hptflen: HPTFLEN,
    #[doc = "0x104 - device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
    pub diep1tflen: DIEP1TFLEN,
    #[doc = "0x108 - device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
    pub diep2tflen: DIEP2TFLEN,
    #[doc = "0x10c - device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
    pub diep3tflen: DIEP3TFLEN,
}
impl RegisterBlock {
    #[doc = "0x1c - Global Receive status read(Host mode)"]
    #[inline(always)]
    pub const fn grstatr_host(&self) -> &GRSTATR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - Global Receive status read(Device mode)"]
    #[inline(always)]
    pub const fn grstatr_device(&self) -> &GRSTATR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x20 - Global Receive status pop(Host mode)"]
    #[inline(always)]
    pub const fn grstatp_host(&self) -> &GRSTATP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x20 - Global Receive status pop(Device mode)"]
    #[inline(always)]
    pub const fn grstatp_device(&self) -> &GRSTATP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    #[doc = "0x28 - Device IN endpoint 0 transmit FIFO length (Device mode)"]
    #[inline(always)]
    pub const fn diep0tflen(&self) -> &DIEP0TFLEN {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - Host non-periodic transmit FIFO length register (Host mode)"]
    #[inline(always)]
    pub const fn hnptflen(&self) -> &HNPTFLEN {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
}
#[doc = "GOTGCS (rw) register accessor: an alias for `Reg<GOTGCS_SPEC>`"]
pub type GOTGCS = crate::Reg<gotgcs::GOTGCS_SPEC>;
#[doc = "Global OTG control and status register (USBFS_GOTGCS)"]
pub mod gotgcs;
#[doc = "GOTGINTF (rw) register accessor: an alias for `Reg<GOTGINTF_SPEC>`"]
pub type GOTGINTF = crate::Reg<gotgintf::GOTGINTF_SPEC>;
#[doc = "Global OTG interrupt flag register (USBFS_GOTGINTF)"]
pub mod gotgintf;
#[doc = "GAHBCS (rw) register accessor: an alias for `Reg<GAHBCS_SPEC>`"]
pub type GAHBCS = crate::Reg<gahbcs::GAHBCS_SPEC>;
#[doc = "Global AHB control and status register (USBFS_GAHBCS)"]
pub mod gahbcs;
#[doc = "GUSBCS (rw) register accessor: an alias for `Reg<GUSBCS_SPEC>`"]
pub type GUSBCS = crate::Reg<gusbcs::GUSBCS_SPEC>;
#[doc = "Global USB control and status register (USBFS_GUSBCSR)"]
pub mod gusbcs;
#[doc = "GRSTCTL (rw) register accessor: an alias for `Reg<GRSTCTL_SPEC>`"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Global reset control register (USBFS_GRSTCTL)"]
pub mod grstctl;
#[doc = "GINTF (rw) register accessor: an alias for `Reg<GINTF_SPEC>`"]
pub type GINTF = crate::Reg<gintf::GINTF_SPEC>;
#[doc = "Global interrupt flag register (USBFS_GINTF)"]
pub mod gintf;
#[doc = "GINTEN (rw) register accessor: an alias for `Reg<GINTEN_SPEC>`"]
pub type GINTEN = crate::Reg<ginten::GINTEN_SPEC>;
#[doc = "Global interrupt enable register (USBFS_GINTEN)"]
pub mod ginten;
#[doc = "GRSTATR_Device (r) register accessor: an alias for `Reg<GRSTATR_DEVICE_SPEC>`"]
pub type GRSTATR_DEVICE = crate::Reg<grstatr_device::GRSTATR_DEVICE_SPEC>;
#[doc = "Global Receive status read(Device mode)"]
pub mod grstatr_device;
#[doc = "GRSTATR_Host (r) register accessor: an alias for `Reg<GRSTATR_HOST_SPEC>`"]
pub type GRSTATR_HOST = crate::Reg<grstatr_host::GRSTATR_HOST_SPEC>;
#[doc = "Global Receive status read(Host mode)"]
pub mod grstatr_host;
#[doc = "GRSTATP_Device (r) register accessor: an alias for `Reg<GRSTATP_DEVICE_SPEC>`"]
pub type GRSTATP_DEVICE = crate::Reg<grstatp_device::GRSTATP_DEVICE_SPEC>;
#[doc = "Global Receive status pop(Device mode)"]
pub mod grstatp_device;
#[doc = "GRSTATP_Host (r) register accessor: an alias for `Reg<GRSTATP_HOST_SPEC>`"]
pub type GRSTATP_HOST = crate::Reg<grstatp_host::GRSTATP_HOST_SPEC>;
#[doc = "Global Receive status pop(Host mode)"]
pub mod grstatp_host;
#[doc = "GRFLEN (rw) register accessor: an alias for `Reg<GRFLEN_SPEC>`"]
pub type GRFLEN = crate::Reg<grflen::GRFLEN_SPEC>;
#[doc = "Global Receive FIFO size register (USBFS_GRFLEN)"]
pub mod grflen;
#[doc = "HNPTFLEN (rw) register accessor: an alias for `Reg<HNPTFLEN_SPEC>`"]
pub type HNPTFLEN = crate::Reg<hnptflen::HNPTFLEN_SPEC>;
#[doc = "Host non-periodic transmit FIFO length register (Host mode)"]
pub mod hnptflen;
#[doc = "DIEP0TFLEN (rw) register accessor: an alias for `Reg<DIEP0TFLEN_SPEC>`"]
pub type DIEP0TFLEN = crate::Reg<diep0tflen::DIEP0TFLEN_SPEC>;
#[doc = "Device IN endpoint 0 transmit FIFO length (Device mode)"]
pub mod diep0tflen;
#[doc = "HNPTFQSTAT (r) register accessor: an alias for `Reg<HNPTFQSTAT_SPEC>`"]
pub type HNPTFQSTAT = crate::Reg<hnptfqstat::HNPTFQSTAT_SPEC>;
#[doc = "Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
pub mod hnptfqstat;
#[doc = "GCCFG (rw) register accessor: an alias for `Reg<GCCFG_SPEC>`"]
pub type GCCFG = crate::Reg<gccfg::GCCFG_SPEC>;
#[doc = "Global core configuration register (USBFS_GCCFG)"]
pub mod gccfg;
#[doc = "CID (rw) register accessor: an alias for `Reg<CID_SPEC>`"]
pub type CID = crate::Reg<cid::CID_SPEC>;
#[doc = "core ID register"]
pub mod cid;
#[doc = "HPTFLEN (rw) register accessor: an alias for `Reg<HPTFLEN_SPEC>`"]
pub type HPTFLEN = crate::Reg<hptflen::HPTFLEN_SPEC>;
#[doc = "Host periodic transmit FIFO length register (HPTFLEN)"]
pub mod hptflen;
#[doc = "DIEP1TFLEN (rw) register accessor: an alias for `Reg<DIEP1TFLEN_SPEC>`"]
pub type DIEP1TFLEN = crate::Reg<diep1tflen::DIEP1TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
pub mod diep1tflen;
#[doc = "DIEP2TFLEN (rw) register accessor: an alias for `Reg<DIEP2TFLEN_SPEC>`"]
pub type DIEP2TFLEN = crate::Reg<diep2tflen::DIEP2TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
pub mod diep2tflen;
#[doc = "DIEP3TFLEN (rw) register accessor: an alias for `Reg<DIEP3TFLEN_SPEC>`"]
pub type DIEP3TFLEN = crate::Reg<diep3tflen::DIEP3TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
pub mod diep3tflen;
