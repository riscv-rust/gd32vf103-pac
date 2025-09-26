#[doc = "Register `CH1CV` reader"]
pub type R = crate::R<CH1CV_SPEC>;
#[doc = "Register `CH1CV` writer"]
pub type W = crate::W<CH1CV_SPEC>;
#[doc = "Field `CH1VAL` reader - Capture or compare value of channel1"]
pub type CH1VAL_R = crate::FieldReader<u16>;
#[doc = "Field `CH1VAL` writer - Capture or compare value of channel1"]
pub type CH1VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
    pub fn ch1val(&mut self) -> CH1VAL_W<CH1CV_SPEC, 0> {
        CH1VAL_W::new(self)
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
#[doc = "Channel 1 capture/compare value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1CV_SPEC;
impl crate::RegisterSpec for CH1CV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ch1cv::R`](R) reader structure"]
impl crate::Readable for CH1CV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch1cv::W`](W) writer structure"]
impl crate::Writable for CH1CV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1CV to value 0"]
impl crate::Resettable for CH1CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
