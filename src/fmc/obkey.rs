#[doc = "Register `OBKEY` writer"]
pub type W = crate::W<OBKEY_SPEC>;
#[doc = "Field `OBKEY` writer - FMC_ CTL0 option byte operation unlock register"]
pub type OBKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - FMC_ CTL0 option byte operation unlock register"]
    #[inline(always)]
    #[must_use]
    pub fn obkey(&mut self) -> OBKEY_W<OBKEY_SPEC, 0> {
        OBKEY_W::new(self)
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
#[doc = "Option byte unlock key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`obkey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OBKEY_SPEC;
impl crate::RegisterSpec for OBKEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`obkey::W`](W) writer structure"]
impl crate::Writable for OBKEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OBKEY to value 0"]
impl crate::Resettable for OBKEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
