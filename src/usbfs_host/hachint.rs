#[doc = "Register `HACHINT` reader"]
pub type R = crate::R<HACHINT_SPEC>;
#[doc = "Field `HACHINT` reader - Host all channel interrupts"]
pub type HACHINT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Host all channel interrupts"]
    #[inline(always)]
    pub fn hachint(&self) -> HACHINT_R {
        HACHINT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Host all channels interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hachint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HACHINT_SPEC;
impl crate::RegisterSpec for HACHINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hachint::R`](R) reader structure"]
impl crate::Readable for HACHINT_SPEC {}
#[doc = "`reset()` method sets HACHINT to value 0"]
impl crate::Resettable for HACHINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
