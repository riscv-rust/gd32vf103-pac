#[doc = "Register `CLICCFG` reader"]
pub struct R(crate::R<CLICCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLICCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLICCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLICCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLICCFG` writer"]
pub struct W(crate::W<CLICCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLICCFG_SPEC>;
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
impl From<crate::W<CLICCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLICCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NLBITS` reader - NLBITS"]
pub type NLBITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NLBITS` writer - NLBITS"]
pub type NLBITS_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CLICCFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 1:4 - NLBITS"]
    #[inline(always)]
    pub fn nlbits(&self) -> NLBITS_R {
        NLBITS_R::new((self.bits >> 1) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 1:4 - NLBITS"]
    #[inline(always)]
    #[must_use]
    pub fn nlbits(&mut self) -> NLBITS_W<1> {
        NLBITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cliccfg Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cliccfg](index.html) module"]
pub struct CLICCFG_SPEC;
impl crate::RegisterSpec for CLICCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cliccfg::R](R) reader structure"]
impl crate::Readable for CLICCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cliccfg::W](W) writer structure"]
impl crate::Writable for CLICCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLICCFG to value 0"]
impl crate::Resettable for CLICCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
