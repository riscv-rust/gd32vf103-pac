#[doc = "Register `DOEPINTEN` reader"]
pub type R = crate::R<DOEPINTEN_SPEC>;
#[doc = "Register `DOEPINTEN` writer"]
pub type W = crate::W<DOEPINTEN_SPEC>;
#[doc = "Field `TFEN` reader - Transfer finished interrupt enable"]
pub type TFEN_R = crate::BitReader;
#[doc = "Field `TFEN` writer - Transfer finished interrupt enable"]
pub type TFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDISEN` reader - Endpoint disabled interrupt enable"]
pub type EPDISEN_R = crate::BitReader;
#[doc = "Field `EPDISEN` writer - Endpoint disabled interrupt enable"]
pub type EPDISEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STPFEN` reader - SETUP phase finished interrupt enable"]
pub type STPFEN_R = crate::BitReader;
#[doc = "Field `STPFEN` writer - SETUP phase finished interrupt enable"]
pub type STPFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPRXFOVREN` reader - Endpoint Rx FIFO overrun interrupt enable"]
pub type EPRXFOVREN_R = crate::BitReader;
#[doc = "Field `EPRXFOVREN` writer - Endpoint Rx FIFO overrun interrupt enable"]
pub type EPRXFOVREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BTBSTPEN` reader - Back-to-back SETUP packets interrupt enable"]
pub type BTBSTPEN_R = crate::BitReader;
#[doc = "Field `BTBSTPEN` writer - Back-to-back SETUP packets interrupt enable"]
pub type BTBSTPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn tfen(&mut self) -> TFEN_W<DOEPINTEN_SPEC, 0> {
        TFEN_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epdisen(&mut self) -> EPDISEN_W<DOEPINTEN_SPEC, 1> {
        EPDISEN_W::new(self)
    }
    #[doc = "Bit 3 - SETUP phase finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stpfen(&mut self) -> STPFEN_W<DOEPINTEN_SPEC, 3> {
        STPFEN_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eprxfovren(&mut self) -> EPRXFOVREN_W<DOEPINTEN_SPEC, 4> {
        EPRXFOVREN_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn btbstpen(&mut self) -> BTBSTPEN_W<DOEPINTEN_SPEC, 6> {
        BTBSTPEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "device OUT endpoint common interrupt enable register (DOEPINTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepinten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepinten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINTEN_SPEC;
impl crate::RegisterSpec for DOEPINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepinten::R`](R) reader structure"]
impl crate::Readable for DOEPINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepinten::W`](W) writer structure"]
impl crate::Writable for DOEPINTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPINTEN to value 0"]
impl crate::Resettable for DOEPINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
