#[doc = "Register `CH3CV` reader"]
pub type R = crate::R<CH3CV_SPEC>;
#[doc = "Register `CH3CV` writer"]
pub type W = crate::W<CH3CV_SPEC>;
#[doc = "Field `CH3VAL` reader - Capture or compare value of channel 3"]
pub type CH3VAL_R = crate::FieldReader<u16>;
#[doc = "Field `CH3VAL` writer - Capture or compare value of channel 3"]
pub type CH3VAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
    pub fn ch3val(&mut self) -> CH3VAL_W<CH3CV_SPEC, 0> {
        CH3VAL_W::new(self)
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
#[doc = "Channel 3 capture/compare value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3CV_SPEC;
impl crate::RegisterSpec for CH3CV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ch3cv::R`](R) reader structure"]
impl crate::Readable for CH3CV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch3cv::W`](W) writer structure"]
impl crate::Writable for CH3CV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3CV to value 0"]
impl crate::Resettable for CH3CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
