#[doc = "Register `CH0CV` reader"]
pub type R = crate::R<CH0CV_SPEC>;
#[doc = "Register `CH0CV` writer"]
pub type W = crate::W<CH0CV_SPEC>;
#[doc = "Field `CH0VAL` reader - Capture or compare value of channel0"]
pub type CH0VAL_R = crate::FieldReader<u16>;
#[doc = "Field `CH0VAL` writer - Capture or compare value of channel0"]
pub type CH0VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
    pub fn ch0val(&mut self) -> CH0VAL_W<CH0CV_SPEC, 0> {
        CH0VAL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel 0 capture/compare value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH0CV_SPEC;
impl crate::RegisterSpec for CH0CV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ch0cv::R`](R) reader structure"]
impl crate::Readable for CH0CV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch0cv::W`](W) writer structure"]
impl crate::Writable for CH0CV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH0CV to value 0"]
impl crate::Resettable for CH0CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
