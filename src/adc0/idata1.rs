#[doc = "Register `IDATA1` reader"]
pub type R = crate::R<IDATA1_SPEC>;
#[doc = "Field `IDATAn` reader - Inserted number n conversion data"]
pub type IDATAN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Inserted number n conversion data"]
    #[inline(always)]
    pub fn idatan(&self) -> IDATAN_R {
        IDATAN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Inserted data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`idata1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDATA1_SPEC;
impl crate::RegisterSpec for IDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idata1::R`](R) reader structure"]
impl crate::Readable for IDATA1_SPEC {}
#[doc = "`reset()` method sets IDATA1 to value 0"]
impl crate::Resettable for IDATA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
