#[doc = "Register `IDATA1` reader"]
pub struct R(crate::R<IDATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDATA1_SPEC>) -> Self {
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
#[doc = "Inserted data register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idata1](index.html) module"]
pub struct IDATA1_SPEC;
impl crate::RegisterSpec for IDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idata1::R](R) reader structure"]
impl crate::Readable for IDATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDATA1 to value 0"]
impl crate::Resettable for IDATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
