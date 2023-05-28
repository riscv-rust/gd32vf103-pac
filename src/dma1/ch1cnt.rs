#[doc = "Register `CH1CNT` reader"]
pub struct R(crate::R<CH1CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1CNT` writer"]
pub struct W(crate::W<CH1CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1CNT_SPEC>;
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
impl From<crate::W<CH1CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Transfer counter"]
pub type CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT` writer - Transfer counter"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1CNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Transfer counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transfer counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 1 counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cnt](index.html) module"]
pub struct CH1CNT_SPEC;
impl crate::RegisterSpec for CH1CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1cnt::R](R) reader structure"]
impl crate::Readable for CH1CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1cnt::W](W) writer structure"]
impl crate::Writable for CH1CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1CNT to value 0"]
impl crate::Resettable for CH1CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
