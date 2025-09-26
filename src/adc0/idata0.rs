#[doc = "Register `IDATA0` reader"]
pub type R = crate::R<IDATA0_SPEC>;
#[doc = "Field `IDATAn` reader - Inserted number n conversion data"]
pub type IDATAN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Inserted number n conversion data"]
    #[inline(always)]
    pub fn idatan(&self) -> IDATAN_R {
        IDATAN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Inserted data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idata0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDATA0_SPEC;
impl crate::RegisterSpec for IDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idata0::R`](R) reader structure"]
impl crate::Readable for IDATA0_SPEC {}
#[doc = "`reset()` method sets IDATA0 to value 0"]
impl crate::Resettable for IDATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
