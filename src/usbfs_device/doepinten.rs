#[doc = "Register `DOEPINTEN` reader"]
pub struct R(crate::R<DOEPINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPINTEN` writer"]
pub struct W(crate::W<DOEPINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPINTEN_SPEC>;
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
impl From<crate::W<DOEPINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFEN` reader - Transfer finished interrupt enable"]
pub type TFEN_R = crate::BitReader<bool>;
#[doc = "Field `TFEN` writer - Transfer finished interrupt enable"]
pub type TFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINTEN_SPEC, bool, O>;
#[doc = "Field `EPDISEN` reader - Endpoint disabled interrupt enable"]
pub type EPDISEN_R = crate::BitReader<bool>;
#[doc = "Field `EPDISEN` writer - Endpoint disabled interrupt enable"]
pub type EPDISEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINTEN_SPEC, bool, O>;
#[doc = "Field `STPFEN` reader - SETUP phase finished interrupt enable"]
pub type STPFEN_R = crate::BitReader<bool>;
#[doc = "Field `STPFEN` writer - SETUP phase finished interrupt enable"]
pub type STPFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINTEN_SPEC, bool, O>;
#[doc = "Field `EPRXFOVREN` reader - Endpoint Rx FIFO overrun interrupt enable"]
pub type EPRXFOVREN_R = crate::BitReader<bool>;
#[doc = "Field `EPRXFOVREN` writer - Endpoint Rx FIFO overrun interrupt enable"]
pub type EPRXFOVREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINTEN_SPEC, bool, O>;
#[doc = "Field `BTBSTPEN` reader - Back-to-back SETUP packets interrupt enable"]
pub type BTBSTPEN_R = crate::BitReader<bool>;
#[doc = "Field `BTBSTPEN` writer - Back-to-back SETUP packets interrupt enable"]
pub type BTBSTPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer finished interrupt enable"]
    #[inline(always)]
    pub fn tfen(&self) -> TFEN_R {
        TFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable"]
    #[inline(always)]
    pub fn epdisen(&self) -> EPDISEN_R {
        EPDISEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase finished interrupt enable"]
    #[inline(always)]
    pub fn stpfen(&self) -> STPFEN_R {
        STPFEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn eprxfovren(&self) -> EPRXFOVREN_R {
        EPRXFOVREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets interrupt enable"]
    #[inline(always)]
    pub fn btbstpen(&self) -> BTBSTPEN_R {
        BTBSTPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfen(&mut self) -> TFEN_W<0> {
        TFEN_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epdisen(&mut self) -> EPDISEN_W<1> {
        EPDISEN_W::new(self)
    }
    #[doc = "Bit 3 - SETUP phase finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stpfen(&mut self) -> STPFEN_W<3> {
        STPFEN_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eprxfovren(&mut self) -> EPRXFOVREN_W<4> {
        EPRXFOVREN_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn btbstpen(&mut self) -> BTBSTPEN_W<6> {
        BTBSTPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device OUT endpoint common interrupt enable register (DOEPINTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepinten](index.html) module"]
pub struct DOEPINTEN_SPEC;
impl crate::RegisterSpec for DOEPINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepinten::R](R) reader structure"]
impl crate::Readable for DOEPINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepinten::W](W) writer structure"]
impl crate::Writable for DOEPINTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPINTEN to value 0"]
impl crate::Resettable for DOEPINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
