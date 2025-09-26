#[doc = "Register `HPTFQSTAT` reader"]
pub type R = crate::R<HPTFQSTAT_SPEC>;
#[doc = "Field `PTXFS` reader - Periodic transmit data FIFO space available"]
pub type PTXFS_R = crate::FieldReader<u16>;
#[doc = "Field `PTXREQS` reader - Periodic transmit request queue space available"]
pub type PTXREQS_R = crate::FieldReader;
#[doc = "Field `PTXREQT` reader - Top of the periodic transmit request queue"]
pub type PTXREQT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    pub fn ptxfs(&self) -> PTXFS_R {
        PTXFS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic transmit request queue space available"]
    #[inline(always)]
    pub fn ptxreqs(&self) -> PTXREQS_R {
        PTXREQS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the periodic transmit request queue"]
    #[inline(always)]
    pub fn ptxreqt(&self) -> PTXREQT_R {
        PTXREQT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Host periodic transmit FIFO/queue status register (HPTFQSTAT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptfqstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTFQSTAT_SPEC;
impl crate::RegisterSpec for HPTFQSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptfqstat::R`](R) reader structure"]
impl crate::Readable for HPTFQSTAT_SPEC {}
#[doc = "`reset()` method sets HPTFQSTAT to value 0x0008_0200"]
impl crate::Resettable for HPTFQSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0200;
}
