#[doc = "Register `CH0CV` reader"]
pub struct R(crate::R<CH0CV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0CV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0CV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0CV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH0CV` writer"]
pub struct W(crate::W<CH0CV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH0CV_SPEC>;
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
impl From<crate::W<CH0CV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH0CV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0VAL` reader - Capture or compare value of channel0"]
pub type CH0VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH0VAL` writer - Capture or compare value of channel0"]
pub type CH0VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CH0CV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture or compare value of channel0"]
    #[inline(always)]
    pub fn ch0val(&self) -> CH0VAL_R {
        CH0VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value of channel0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0val(&mut self) -> CH0VAL_W<0> {
        CH0VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 capture/compare value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0cv](index.html) module"]
pub struct CH0CV_SPEC;
impl crate::RegisterSpec for CH0CV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ch0cv::R](R) reader structure"]
impl crate::Readable for CH0CV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch0cv::W](W) writer structure"]
impl crate::Writable for CH0CV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH0CV to value 0"]
impl crate::Resettable for CH0CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
