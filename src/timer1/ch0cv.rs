#[doc = "Register `CH0CV` reader"]
pub type R = crate::R<CH0CV_SPEC>;
#[doc = "Register `CH0CV` writer"]
pub type W = crate::W<CH0CV_SPEC>;
#[doc = "Field `CH0VAL` reader - Capture or compare value of channel 0"]
pub type CH0VAL_R = crate::FieldReader<u16>;
#[doc = "Field `CH0VAL` writer - Capture or compare value of channel 0"]
pub type CH0VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture or compare value of channel 0"]
    #[inline(always)]
    pub fn ch0val(&self) -> CH0VAL_R {
        CH0VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value of channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0val(&mut self) -> CH0VAL_W<CH0CV_SPEC> {
        CH0VAL_W::new(self, 0)
    }
}
#[doc = "Channel 0 capture/compare value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH0CV_SPEC;
impl crate::RegisterSpec for CH0CV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ch0cv::R`](R) reader structure"]
impl crate::Readable for CH0CV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch0cv::W`](W) writer structure"]
impl crate::Writable for CH0CV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CH0CV to value 0"]
impl crate::Resettable for CH0CV_SPEC {
    const RESET_VALUE: u16 = 0;
}
