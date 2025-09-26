#[doc = "Register `DIEPINTEN` reader"]
pub type R = crate::R<DIEPINTEN_SPEC>;
#[doc = "Register `DIEPINTEN` writer"]
pub type W = crate::W<DIEPINTEN_SPEC>;
#[doc = "Field `TFEN` reader - Transfer finished interrupt enable"]
pub type TFEN_R = crate::BitReader;
#[doc = "Field `TFEN` writer - Transfer finished interrupt enable"]
pub type TFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDISEN` reader - Endpoint disabled interrupt enable"]
pub type EPDISEN_R = crate::BitReader;
#[doc = "Field `EPDISEN` writer - Endpoint disabled interrupt enable"]
pub type EPDISEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CITOEN` reader - Control IN timeout condition interrupt enable (Non-isochronous endpoints)"]
pub type CITOEN_R = crate::BitReader;
#[doc = "Field `CITOEN` writer - Control IN timeout condition interrupt enable (Non-isochronous endpoints)"]
pub type CITOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPTXFUDEN` reader - Endpoint Tx FIFO underrun interrupt enable bit"]
pub type EPTXFUDEN_R = crate::BitReader;
#[doc = "Field `EPTXFUDEN` writer - Endpoint Tx FIFO underrun interrupt enable bit"]
pub type EPTXFUDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IEPNEEN` reader - IN endpoint NAK effective interrupt enable"]
pub type IEPNEEN_R = crate::BitReader;
#[doc = "Field `IEPNEEN` writer - IN endpoint NAK effective interrupt enable"]
pub type IEPNEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 3 - Control IN timeout condition interrupt enable (Non-isochronous endpoints)"]
    #[inline(always)]
    pub fn citoen(&self) -> CITOEN_R {
        CITOEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Tx FIFO underrun interrupt enable bit"]
    #[inline(always)]
    pub fn eptxfuden(&self) -> EPTXFUDEN_R {
        EPTXFUDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective interrupt enable"]
    #[inline(always)]
    pub fn iepneen(&self) -> IEPNEEN_R {
        IEPNEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfen(&mut self) -> TFEN_W<DIEPINTEN_SPEC, 0> {
        TFEN_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epdisen(&mut self) -> EPDISEN_W<DIEPINTEN_SPEC, 1> {
        EPDISEN_W::new(self)
    }
    #[doc = "Bit 3 - Control IN timeout condition interrupt enable (Non-isochronous endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn citoen(&mut self) -> CITOEN_W<DIEPINTEN_SPEC, 3> {
        CITOEN_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint Tx FIFO underrun interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn eptxfuden(&mut self) -> EPTXFUDEN_W<DIEPINTEN_SPEC, 4> {
        EPTXFUDEN_W::new(self)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iepneen(&mut self) -> IEPNEEN_W<DIEPINTEN_SPEC, 6> {
        IEPNEEN_W::new(self)
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
#[doc = "device IN endpoint common interrupt mask register (DIEPINTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepinten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepinten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPINTEN_SPEC;
impl crate::RegisterSpec for DIEPINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepinten::R`](R) reader structure"]
impl crate::Readable for DIEPINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepinten::W`](W) writer structure"]
impl crate::Writable for DIEPINTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINTEN to value 0"]
impl crate::Resettable for DIEPINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
