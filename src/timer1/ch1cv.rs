#[doc = "Register `CH1CV` reader"]
pub type R = crate::R<CH1CV_SPEC>;
#[doc = "Register `CH1CV` writer"]
pub type W = crate::W<CH1CV_SPEC>;
#[doc = "Field `CH1VAL` reader - Capture or compare value of channel1"]
pub type CH1VAL_R = crate::FieldReader<u16>;
#[doc = "Field `CH1VAL` writer - Capture or compare value of channel1"]
pub type CH1VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn ch1val(&mut self) -> CH1VAL_W<CH1CV_SPEC> {
        CH1VAL_W::new(self, 0)
    }
}
#[doc = "Channel 1 capture/compare value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1CV_SPEC;
impl crate::RegisterSpec for CH1CV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ch1cv::R`](R) reader structure"]
impl crate::Readable for CH1CV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch1cv::W`](W) writer structure"]
impl crate::Writable for CH1CV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CH1CV to value 0"]
impl crate::Resettable for CH1CV_SPEC {
    const RESET_VALUE: u16 = 0;
}
