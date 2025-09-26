#[doc = "Register `DIEP2TFSTAT` reader"]
pub type R = crate::R<DIEP2TFSTAT_SPEC>;
#[doc = "Field `IEPTFS` reader - IN endpoint TxFIFO space remaining"]
pub type IEPTFS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space remaining"]
    #[inline(always)]
    pub fn ieptfs(&self) -> IEPTFS_R {
        IEPTFS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "device IN endpoint 2 transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`diep2tfstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP2TFSTAT_SPEC;
impl crate::RegisterSpec for DIEP2TFSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep2tfstat::R`](R) reader structure"]
impl crate::Readable for DIEP2TFSTAT_SPEC {}
#[doc = "`reset()` method sets DIEP2TFSTAT to value 0x0200"]
impl crate::Resettable for DIEP2TFSTAT_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
