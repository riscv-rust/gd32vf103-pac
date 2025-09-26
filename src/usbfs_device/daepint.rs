#[doc = "Register `DAEPINT` reader"]
pub type R = crate::R<DAEPINT_SPEC>;
#[doc = "Field `IEPITB` reader - Device all IN endpoint interrupt bits"]
pub type IEPITB_R = crate::FieldReader;
#[doc = "Field `OEPITB` reader - Device all OUT endpoint interrupt bits"]
pub type OEPITB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Device all IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepitb(&self) -> IEPITB_R {
        IEPITB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Device all OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepitb(&self) -> OEPITB_R {
        OEPITB_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "device all endpoints interrupt register (DAEPINT)\n\nYou can [`read`](crate::Reg::read) this register and get [`daepint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAEPINT_SPEC;
impl crate::RegisterSpec for DAEPINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daepint::R`](R) reader structure"]
impl crate::Readable for DAEPINT_SPEC {}
#[doc = "`reset()` method sets DAEPINT to value 0"]
impl crate::Resettable for DAEPINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
