#[doc = "Register `CH1CV` reader"]
pub struct R(crate::R<CH1CV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1CV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1CV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1CV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1CV` writer"]
pub struct W(crate::W<CH1CV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1CV_SPEC>;
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
impl From<crate::W<CH1CV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1CV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1VAL` reader - Capture or compare value of channel1"]
pub type CH1VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH1VAL` writer - Capture or compare value of channel1"]
pub type CH1VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CH1CV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture or compare value of channel1"]
    #[inline(always)]
    pub fn ch1val(&self) -> CH1VAL_R {
        CH1VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value of channel1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1val(&mut self) -> CH1VAL_W<0> {
        CH1VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 1 capture/compare value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cv](index.html) module"]
pub struct CH1CV_SPEC;
impl crate::RegisterSpec for CH1CV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ch1cv::R](R) reader structure"]
impl crate::Readable for CH1CV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1cv::W](W) writer structure"]
impl crate::Writable for CH1CV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1CV to value 0"]
impl crate::Resettable for CH1CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
