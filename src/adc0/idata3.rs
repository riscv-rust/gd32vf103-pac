#[doc = "Register `IDATA3` reader"]
pub struct R(crate::R<IDATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDATAn` reader - Inserted number n conversion data"]
pub type IDATAN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Inserted number n conversion data"]
    #[inline(always)]
    pub fn idatan(&self) -> IDATAN_R {
        IDATAN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Inserted data register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idata3](index.html) module"]
pub struct IDATA3_SPEC;
impl crate::RegisterSpec for IDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idata3::R](R) reader structure"]
impl crate::Readable for IDATA3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDATA3 to value 0"]
impl crate::Resettable for IDATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
