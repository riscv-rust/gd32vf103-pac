#[doc = "Register `OBKEY` writer"]
pub struct W(crate::W<OBKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OBKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OBKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OBKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OBKEY` writer - FMC_ CTL0 option byte operation unlock register"]
pub type OBKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OBKEY_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - FMC_ CTL0 option byte operation unlock register"]
    #[inline(always)]
    #[must_use]
    pub fn obkey(&mut self) -> OBKEY_W<0> {
        OBKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Option byte unlock key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obkey](index.html) module"]
pub struct OBKEY_SPEC;
impl crate::RegisterSpec for OBKEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [obkey::W](W) writer structure"]
impl crate::Writable for OBKEY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OBKEY to value 0"]
impl crate::Resettable for OBKEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
