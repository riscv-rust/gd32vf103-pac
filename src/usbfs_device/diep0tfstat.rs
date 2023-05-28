#[doc = "Register `DIEP0TFSTAT` reader"]
pub struct R(crate::R<DIEP0TFSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP0TFSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP0TFSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP0TFSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IEPTFS` reader - IN endpoint TxFIFO space remaining"]
pub type IEPTFS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space remaining"]
    #[inline(always)]
    pub fn ieptfs(&self) -> IEPTFS_R {
        IEPTFS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "device IN endpoint 0 transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0tfstat](index.html) module"]
pub struct DIEP0TFSTAT_SPEC;
impl crate::RegisterSpec for DIEP0TFSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep0tfstat::R](R) reader structure"]
impl crate::Readable for DIEP0TFSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIEP0TFSTAT to value 0x0200"]
impl crate::Resettable for DIEP0TFSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
