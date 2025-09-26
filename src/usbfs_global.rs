#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gotgcs: GOTGCS,
    gotgintf: GOTGINTF,
    gahbcs: GAHBCS,
    gusbcs: GUSBCS,
    grstctl: GRSTCTL,
    gintf: GINTF,
    ginten: GINTEN,
    _reserved_7_grstatr: [u8; 0x04],
    _reserved_8_grstatp: [u8; 0x04],
    grflen: GRFLEN,
    _reserved_10_hnptflen: [u8; 0x04],
    hnptfqstat: HNPTFQSTAT,
    _reserved12: [u8; 0x08],
    gccfg: GCCFG,
    cid: CID,
    _reserved14: [u8; 0xc0],
    hptflen: HPTFLEN,
    diep1tflen: DIEP1TFLEN,
    diep2tflen: DIEP2TFLEN,
    diep3tflen: DIEP3TFLEN,
}
impl RegisterBlock {
    #[doc = "0x00 - Global OTG control and status register (USBFS_GOTGCS)"]
    #[inline(always)]
    pub const fn gotgcs(&self) -> &GOTGCS {
        &self.gotgcs
    }
    #[doc = "0x04 - Global OTG interrupt flag register (USBFS_GOTGINTF)"]
    #[inline(always)]
    pub const fn gotgintf(&self) -> &GOTGINTF {
        &self.gotgintf
    }
    #[doc = "0x08 - Global AHB control and status register (USBFS_GAHBCS)"]
    #[inline(always)]
    pub const fn gahbcs(&self) -> &GAHBCS {
        &self.gahbcs
    }
    #[doc = "0x0c - Global USB control and status register (USBFS_GUSBCSR)"]
    #[inline(always)]
    pub const fn gusbcs(&self) -> &GUSBCS {
        &self.gusbcs
    }
    #[doc = "0x10 - Global reset control register (USBFS_GRSTCTL)"]
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    #[doc = "0x14 - Global interrupt flag register (USBFS_GINTF)"]
    #[inline(always)]
    pub const fn gintf(&self) -> &GINTF {
        &self.gintf
    }
    #[doc = "0x18 - Global interrupt enable register (USBFS_GINTEN)"]
    #[inline(always)]
    pub const fn ginten(&self) -> &GINTEN {
        &self.ginten
    }
    #[doc = "0x1c - Global Receive status read(Host mode)"]
    #[inline(always)]
    pub const fn grstatr_host(&self) -> &GRSTATR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Global Receive status read(Device mode)"]
    #[inline(always)]
    pub const fn grstatr_device(&self) -> &GRSTATR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - Global Receive status pop(Host mode)"]
    #[inline(always)]
    pub const fn grstatp_host(&self) -> &GRSTATP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - Global Receive status pop(Device mode)"]
    #[inline(always)]
    pub const fn grstatp_device(&self) -> &GRSTATP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - Global Receive FIFO size register (USBFS_GRFLEN)"]
    #[inline(always)]
    pub const fn grflen(&self) -> &GRFLEN {
        &self.grflen
    }
    #[doc = "0x28 - Device IN endpoint 0 transmit FIFO length (Device mode)"]
    #[inline(always)]
    pub const fn diep0tflen(&self) -> &DIEP0TFLEN {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - Host non-periodic transmit FIFO length register (Host mode)"]
    #[inline(always)]
    pub const fn hnptflen(&self) -> &HNPTFLEN {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
    #[inline(always)]
    pub const fn hnptfqstat(&self) -> &HNPTFQSTAT {
        &self.hnptfqstat
    }
    #[doc = "0x38 - Global core configuration register (USBFS_GCCFG)"]
    #[inline(always)]
    pub const fn gccfg(&self) -> &GCCFG {
        &self.gccfg
    }
    #[doc = "0x3c - core ID register"]
    #[inline(always)]
    pub const fn cid(&self) -> &CID {
        &self.cid
    }
    #[doc = "0x100 - Host periodic transmit FIFO length register (HPTFLEN)"]
    #[inline(always)]
    pub const fn hptflen(&self) -> &HPTFLEN {
        &self.hptflen
    }
    #[doc = "0x104 - device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
    #[inline(always)]
    pub const fn diep1tflen(&self) -> &DIEP1TFLEN {
        &self.diep1tflen
    }
    #[doc = "0x108 - device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
    #[inline(always)]
    pub const fn diep2tflen(&self) -> &DIEP2TFLEN {
        &self.diep2tflen
    }
    #[doc = "0x10c - device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
    #[inline(always)]
    pub const fn diep3tflen(&self) -> &DIEP3TFLEN {
        &self.diep3tflen
    }
}
#[doc = "GOTGCS (rw) register accessor: Global OTG control and status register (USBFS_GOTGCS)\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgcs`]
module"]
pub type GOTGCS = crate::Reg<gotgcs::GOTGCS_SPEC>;
#[doc = "Global OTG control and status register (USBFS_GOTGCS)"]
pub mod gotgcs;
#[doc = "GOTGINTF (rw) register accessor: Global OTG interrupt flag register (USBFS_GOTGINTF)\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgintf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgintf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgintf`]
module"]
pub type GOTGINTF = crate::Reg<gotgintf::GOTGINTF_SPEC>;
#[doc = "Global OTG interrupt flag register (USBFS_GOTGINTF)"]
pub mod gotgintf;
#[doc = "GAHBCS (rw) register accessor: Global AHB control and status register (USBFS_GAHBCS)\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcs`]
module"]
pub type GAHBCS = crate::Reg<gahbcs::GAHBCS_SPEC>;
#[doc = "Global AHB control and status register (USBFS_GAHBCS)"]
pub mod gahbcs;
#[doc = "GUSBCS (rw) register accessor: Global USB control and status register (USBFS_GUSBCSR)\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcs`]
module"]
pub type GUSBCS = crate::Reg<gusbcs::GUSBCS_SPEC>;
#[doc = "Global USB control and status register (USBFS_GUSBCSR)"]
pub mod gusbcs;
#[doc = "GRSTCTL (rw) register accessor: Global reset control register (USBFS_GRSTCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`]
module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Global reset control register (USBFS_GRSTCTL)"]
pub mod grstctl;
#[doc = "GINTF (rw) register accessor: Global interrupt flag register (USBFS_GINTF)\n\nYou can [`read`](crate::Reg::read) this register and get [`gintf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintf`]
module"]
pub type GINTF = crate::Reg<gintf::GINTF_SPEC>;
#[doc = "Global interrupt flag register (USBFS_GINTF)"]
pub mod gintf;
#[doc = "GINTEN (rw) register accessor: Global interrupt enable register (USBFS_GINTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`ginten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ginten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ginten`]
module"]
pub type GINTEN = crate::Reg<ginten::GINTEN_SPEC>;
#[doc = "Global interrupt enable register (USBFS_GINTEN)"]
pub mod ginten;
#[doc = "GRSTATR_Device (r) register accessor: Global Receive status read(Device mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grstatr_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstatr_device`]
module"]
#[doc(alias = "GRSTATR_Device")]
pub type GRSTATR_DEVICE = crate::Reg<grstatr_device::GRSTATR_DEVICE_SPEC>;
#[doc = "Global Receive status read(Device mode)"]
pub mod grstatr_device;
#[doc = "GRSTATR_Host (r) register accessor: Global Receive status read(Host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grstatr_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstatr_host`]
module"]
#[doc(alias = "GRSTATR_Host")]
pub type GRSTATR_HOST = crate::Reg<grstatr_host::GRSTATR_HOST_SPEC>;
#[doc = "Global Receive status read(Host mode)"]
pub mod grstatr_host;
#[doc = "GRSTATP_Device (r) register accessor: Global Receive status pop(Device mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grstatp_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstatp_device`]
module"]
#[doc(alias = "GRSTATP_Device")]
pub type GRSTATP_DEVICE = crate::Reg<grstatp_device::GRSTATP_DEVICE_SPEC>;
#[doc = "Global Receive status pop(Device mode)"]
pub mod grstatp_device;
#[doc = "GRSTATP_Host (r) register accessor: Global Receive status pop(Host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grstatp_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstatp_host`]
module"]
#[doc(alias = "GRSTATP_Host")]
pub type GRSTATP_HOST = crate::Reg<grstatp_host::GRSTATP_HOST_SPEC>;
#[doc = "Global Receive status pop(Host mode)"]
pub mod grstatp_host;
#[doc = "GRFLEN (rw) register accessor: Global Receive FIFO size register (USBFS_GRFLEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`grflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grflen`]
module"]
pub type GRFLEN = crate::Reg<grflen::GRFLEN_SPEC>;
#[doc = "Global Receive FIFO size register (USBFS_GRFLEN)"]
pub mod grflen;
#[doc = "HNPTFLEN (rw) register accessor: Host non-periodic transmit FIFO length register (Host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`hnptflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hnptflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hnptflen`]
module"]
pub type HNPTFLEN = crate::Reg<hnptflen::HNPTFLEN_SPEC>;
#[doc = "Host non-periodic transmit FIFO length register (Host mode)"]
pub mod hnptflen;
#[doc = "DIEP0TFLEN (rw) register accessor: Device IN endpoint 0 transmit FIFO length (Device mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`diep0tflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep0tflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep0tflen`]
module"]
pub type DIEP0TFLEN = crate::Reg<diep0tflen::DIEP0TFLEN_SPEC>;
#[doc = "Device IN endpoint 0 transmit FIFO length (Device mode)"]
pub mod diep0tflen;
#[doc = "HNPTFQSTAT (r) register accessor: Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`hnptfqstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hnptfqstat`]
module"]
pub type HNPTFQSTAT = crate::Reg<hnptfqstat::HNPTFQSTAT_SPEC>;
#[doc = "Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
pub mod hnptfqstat;
#[doc = "GCCFG (rw) register accessor: Global core configuration register (USBFS_GCCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`gccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gccfg`]
module"]
pub type GCCFG = crate::Reg<gccfg::GCCFG_SPEC>;
#[doc = "Global core configuration register (USBFS_GCCFG)"]
pub mod gccfg;
#[doc = "CID (rw) register accessor: core ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`cid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid`]
module"]
pub type CID = crate::Reg<cid::CID_SPEC>;
#[doc = "core ID register"]
pub mod cid;
#[doc = "HPTFLEN (rw) register accessor: Host periodic transmit FIFO length register (HPTFLEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`hptflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptflen`]
module"]
pub type HPTFLEN = crate::Reg<hptflen::HPTFLEN_SPEC>;
#[doc = "Host periodic transmit FIFO length register (HPTFLEN)"]
pub mod hptflen;
#[doc = "DIEP1TFLEN (rw) register accessor: device IN endpoint transmit FIFO size register (DIEP1TFLEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`diep1tflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep1tflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep1tflen`]
module"]
pub type DIEP1TFLEN = crate::Reg<diep1tflen::DIEP1TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
pub mod diep1tflen;
#[doc = "DIEP2TFLEN (rw) register accessor: device IN endpoint transmit FIFO size register (DIEP2TFLEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2tflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep2tflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep2tflen`]
module"]
pub type DIEP2TFLEN = crate::Reg<diep2tflen::DIEP2TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
pub mod diep2tflen;
#[doc = "DIEP3TFLEN (rw) register accessor: device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`diep3tflen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diep3tflen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diep3tflen`]
module"]
pub type DIEP3TFLEN = crate::Reg<diep3tflen::DIEP3TFLEN_SPEC>;
#[doc = "device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
pub mod diep3tflen;
