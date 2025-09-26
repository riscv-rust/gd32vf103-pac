#[doc = "Register `ID` reader"]
pub type R = crate::R<ID_SPEC>;
#[doc = "Field `ID_CODE` reader - DBG ID code register"]
pub type ID_CODE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DBG ID code register"]
    #[inline(always)]
    pub fn id_code(&self) -> ID_CODE_R {
        ID_CODE_R::new(self.bits)
    }
}
#[doc = "ID code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for ID_SPEC {}
#[doc = "`reset()` method sets ID to value 0"]
impl crate::Resettable for ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
