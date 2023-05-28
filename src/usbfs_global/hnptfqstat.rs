#[doc = "Register `HNPTFQSTAT` reader"]
pub struct R(crate::R<HNPTFQSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HNPTFQSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HNPTFQSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HNPTFQSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NPTXFS` reader - Non-periodic TxFIFO space"]
pub type NPTXFS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NPTXRQS` reader - Non-periodic transmit request queue space"]
pub type NPTXRQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NPTXRQTOP` reader - Top of the non-periodic transmit request queue"]
pub type NPTXRQTOP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO space"]
    #[inline(always)]
    pub fn nptxfs(&self) -> NPTXFS_R {
        NPTXFS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic transmit request queue space"]
    #[inline(always)]
    pub fn nptxrqs(&self) -> NPTXRQS_R {
        NPTXRQS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the non-periodic transmit request queue"]
    #[inline(always)]
    pub fn nptxrqtop(&self) -> NPTXRQTOP_R {
        NPTXRQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hnptfqstat](index.html) module"]
pub struct HNPTFQSTAT_SPEC;
impl crate::RegisterSpec for HNPTFQSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hnptfqstat::R](R) reader structure"]
impl crate::Readable for HNPTFQSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HNPTFQSTAT to value 0x0008_0200"]
impl crate::Resettable for HNPTFQSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0200;
}
