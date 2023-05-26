#[doc = "Register `CH2CV` reader"]
pub struct R(crate::R<CH2CV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2CV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2CV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2CV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2CV` writer"]
pub struct W(crate::W<CH2CV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2CV_SPEC>;
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
impl From<crate::W<CH2CV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2CV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH2VAL` reader - Capture or compare value of channel 2"]
pub type CH2VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH2VAL` writer - Capture or compare value of channel 2"]
pub type CH2VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CH2CV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture or compare value of channel 2"]
    #[inline(always)]
    pub fn ch2val(&self) -> CH2VAL_R {
        CH2VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value of channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2val(&mut self) -> CH2VAL_W<0> {
        CH2VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 2 capture/compare value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cv](index.html) module"]
pub struct CH2CV_SPEC;
impl crate::RegisterSpec for CH2CV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ch2cv::R](R) reader structure"]
impl crate::Readable for CH2CV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2cv::W](W) writer structure"]
impl crate::Writable for CH2CV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2CV to value 0"]
impl crate::Resettable for CH2CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
