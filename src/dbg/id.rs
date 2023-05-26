#[doc = "Register `ID` reader"]
pub struct R(crate::R<ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID_CODE` reader - DBG ID code register"]
pub type ID_CODE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DBG ID code register"]
    #[inline(always)]
    pub fn id_code(&self) -> ID_CODE_R {
        ID_CODE_R::new(self.bits)
    }
}
#[doc = "ID code register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](index.html) module"]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id::R](R) reader structure"]
impl crate::Readable for ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ID to value 0"]
impl crate::Resettable for ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
