#[doc = "Register `RCRC` reader"]
pub type R = crate::R<RCRC_SPEC>;
#[doc = "Field `RCRC` reader - RX CRC value"]
pub type RCRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - RX CRC value"]
    #[inline(always)]
    pub fn rcrc(&self) -> RCRC_R {
        RCRC_R::new(self.bits)
    }
}
#[doc = "RX CRC register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCRC_SPEC;
impl crate::RegisterSpec for RCRC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rcrc::R`](R) reader structure"]
impl crate::Readable for RCRC_SPEC {}
#[doc = "`reset()` method sets RCRC to value 0"]
impl crate::Resettable for RCRC_SPEC {
    const RESET_VALUE: u16 = 0;
}
