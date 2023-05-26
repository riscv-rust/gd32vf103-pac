#[doc = "Register `WDLT` reader"]
pub struct R(crate::R<WDLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDLT` writer"]
pub struct W(crate::W<WDLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDLT_SPEC>;
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
impl From<crate::W<WDLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDLT` reader - Analog watchdog lower threshold"]
pub type WDLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDLT` writer - Analog watchdog lower threshold"]
pub type WDLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDLT_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn wdlt(&self) -> WDLT_R {
        WDLT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    #[must_use]
    pub fn wdlt(&mut self) -> WDLT_W<0> {
        WDLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watchdog lower threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdlt](index.html) module"]
pub struct WDLT_SPEC;
impl crate::RegisterSpec for WDLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdlt::R](R) reader structure"]
impl crate::Readable for WDLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdlt::W](W) writer structure"]
impl crate::Writable for WDLT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDLT to value 0"]
impl crate::Resettable for WDLT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
