#[doc = "Register `CH3CV` reader"]
pub struct R(crate::R<CH3CV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3CV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3CV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3CV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3CV` writer"]
pub struct W(crate::W<CH3CV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3CV_SPEC>;
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
impl From<crate::W<CH3CV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3CV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH3VAL` reader - Capture or compare value of channel 3"]
pub type CH3VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH3VAL` writer - Capture or compare value of channel 3"]
pub type CH3VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CH3CV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture or compare value of channel 3"]
    #[inline(always)]
    pub fn ch3val(&self) -> CH3VAL_R {
        CH3VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value of channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3val(&mut self) -> CH3VAL_W<0> {
        CH3VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 3 capture/compare value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3cv](index.html) module"]
pub struct CH3CV_SPEC;
impl crate::RegisterSpec for CH3CV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ch3cv::R](R) reader structure"]
impl crate::Readable for CH3CV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3cv::W](W) writer structure"]
impl crate::Writable for CH3CV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3CV to value 0"]
impl crate::Resettable for CH3CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
