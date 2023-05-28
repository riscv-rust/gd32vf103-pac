#[doc = "Register `CH2CNT` reader"]
pub struct R(crate::R<CH2CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2CNT` writer"]
pub struct W(crate::W<CH2CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2CNT_SPEC>;
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
impl From<crate::W<CH2CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Transfer counter"]
pub type CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT` writer - Transfer counter"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH2CNT_SPEC, u16, u16, 16, O>;
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
#[doc = "Channel 2 counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cnt](index.html) module"]
pub struct CH2CNT_SPEC;
impl crate::RegisterSpec for CH2CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2cnt::R](R) reader structure"]
impl crate::Readable for CH2CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2cnt::W](W) writer structure"]
impl crate::Writable for CH2CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2CNT to value 0"]
impl crate::Resettable for CH2CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
