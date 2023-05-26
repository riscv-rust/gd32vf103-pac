#[doc = "Register `mstop` reader"]
pub struct R(crate::R<MSTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mstop` writer"]
pub struct W(crate::W<MSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTOP_SPEC>;
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
impl From<crate::W<MSTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMESTOP` reader - Pause (1) or run (0) the timer"]
pub type TIMESTOP_R = crate::BitReader<bool>;
#[doc = "Field `TIMESTOP` writer - Pause (1) or run (0) the timer"]
pub type TIMESTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTOP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pause (1) or run (0) the timer"]
    #[inline(always)]
    pub fn timestop(&self) -> TIMESTOP_R {
        TIMESTOP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pause (1) or run (0) the timer"]
    #[inline(always)]
    #[must_use]
    pub fn timestop(&mut self) -> TIMESTOP_W<0> {
        TIMESTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstop](index.html) module"]
pub struct MSTOP_SPEC;
impl crate::RegisterSpec for MSTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstop::R](R) reader structure"]
impl crate::Readable for MSTOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstop::W](W) writer structure"]
impl crate::Writable for MSTOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mstop to value 0"]
impl crate::Resettable for MSTOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
