#[doc = "Register `ALRMH` writer"]
pub type W = crate::W<ALRMH_SPEC>;
#[doc = "Field `ALRM` writer - Alarm value high"]
pub type ALRM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - Alarm value high"]
    #[inline(always)]
    #[must_use]
    pub fn alrm(&mut self) -> ALRM_W<ALRMH_SPEC, 0> {
        ALRM_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Alarm high register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRMH_SPEC;
impl crate::RegisterSpec for ALRMH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`alrmh::W`](W) writer structure"]
impl crate::Writable for ALRMH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRMH to value 0xffff"]
impl crate::Resettable for ALRMH_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
