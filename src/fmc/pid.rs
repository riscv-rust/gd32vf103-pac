#[doc = "Register `PID` reader"]
pub type R = crate::R<PID_SPEC>;
#[doc = "Field `PID` reader - Product reserved ID code register"]
pub type PID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Product reserved ID code register"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(self.bits)
    }
}
#[doc = "Product ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PID_SPEC;
impl crate::RegisterSpec for PID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid::R`](R) reader structure"]
impl crate::Readable for PID_SPEC {}
#[doc = "`reset()` method sets PID to value 0"]
impl crate::Resettable for PID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
