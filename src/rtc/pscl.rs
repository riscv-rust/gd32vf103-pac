#[doc = "Register `PSCL` writer"]
pub type W = crate::W<PSCL_SPEC>;
#[doc = "Field `PSC` writer - RTC prescaler value low"]
pub type PSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC prescaler value low"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<PSCL_SPEC, 0> {
        PSC_W::new(self)
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
#[doc = "RTC prescaler low register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSCL_SPEC;
impl crate::RegisterSpec for PSCL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pscl::W`](W) writer structure"]
impl crate::Writable for PSCL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSCL to value 0x8000"]
impl crate::Resettable for PSCL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
