#[doc = "Register `DIEP1TFSTAT` reader"]
pub type R = crate::R<DIEP1TFSTAT_SPEC>;
#[doc = "Field `IEPTFS` reader - IN endpoint TxFIFO space remaining"]
pub type IEPTFS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space remaining"]
    #[inline(always)]
    pub fn ieptfs(&self) -> IEPTFS_R {
        IEPTFS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "device IN endpoint 1 transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep1tfstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP1TFSTAT_SPEC;
impl crate::RegisterSpec for DIEP1TFSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep1tfstat::R`](R) reader structure"]
impl crate::Readable for DIEP1TFSTAT_SPEC {}
#[doc = "`reset()` method sets DIEP1TFSTAT to value 0x0200"]
impl crate::Resettable for DIEP1TFSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
