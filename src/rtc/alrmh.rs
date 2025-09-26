#[doc = "Register `ALRMH` writer"]
pub type W = crate::W<ALRMH_SPEC>;
#[doc = "Field `ALRM` writer - Alarm value high"]
pub type ALRM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Alarm value high"]
    #[inline(always)]
    #[must_use]
    pub fn alrm(&mut self) -> ALRM_W<ALRMH_SPEC> {
        ALRM_W::new(self, 0)
    }
}
#[doc = "Alarm high register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRMH_SPEC;
impl crate::RegisterSpec for ALRMH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`alrmh::W`](W) writer structure"]
impl crate::Writable for ALRMH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRMH to value 0xffff"]
impl crate::Resettable for ALRMH_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
