#[doc = "Register `CH2CV` reader"]
pub type R = crate::R<CH2CV_SPEC>;
#[doc = "Register `CH2CV` writer"]
pub type W = crate::W<CH2CV_SPEC>;
#[doc = "Field `CH2VAL` reader - Capture or compare value of channel 2"]
pub type CH2VAL_R = crate::FieldReader<u16>;
#[doc = "Field `CH2VAL` writer - Capture or compare value of channel 2"]
pub type CH2VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn ch2val(&mut self) -> CH2VAL_W<CH2CV_SPEC> {
        CH2VAL_W::new(self, 0)
    }
}
#[doc = "Channel 2 capture/compare value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2CV_SPEC;
impl crate::RegisterSpec for CH2CV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ch2cv::R`](R) reader structure"]
impl crate::Readable for CH2CV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2cv::W`](W) writer structure"]
impl crate::Writable for CH2CV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CH2CV to value 0"]
impl crate::Resettable for CH2CV_SPEC {
    const RESET_VALUE: u16 = 0;
}
