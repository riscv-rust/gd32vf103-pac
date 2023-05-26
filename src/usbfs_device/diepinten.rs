#[doc = "Register `DIEPINTEN` reader"]
pub struct R(crate::R<DIEPINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINTEN` writer"]
pub struct W(crate::W<DIEPINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINTEN_SPEC>;
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
impl From<crate::W<DIEPINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFEN` reader - Transfer finished interrupt enable"]
pub type TFEN_R = crate::BitReader<bool>;
#[doc = "Field `TFEN` writer - Transfer finished interrupt enable"]
pub type TFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINTEN_SPEC, bool, O>;
#[doc = "Field `EPDISEN` reader - Endpoint disabled interrupt enable"]
pub type EPDISEN_R = crate::BitReader<bool>;
#[doc = "Field `EPDISEN` writer - Endpoint disabled interrupt enable"]
pub type EPDISEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINTEN_SPEC, bool, O>;
#[doc = "Field `CITOEN` reader - Control IN timeout condition interrupt enable (Non-isochronous endpoints)"]
pub type CITOEN_R = crate::BitReader<bool>;
#[doc = "Field `CITOEN` writer - Control IN timeout condition interrupt enable (Non-isochronous endpoints)"]
pub type CITOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINTEN_SPEC, bool, O>;
#[doc = "Field `EPTXFUDEN` reader - Endpoint Tx FIFO underrun interrupt enable bit"]
pub type EPTXFUDEN_R = crate::BitReader<bool>;
#[doc = "Field `EPTXFUDEN` writer - Endpoint Tx FIFO underrun interrupt enable bit"]
pub type EPTXFUDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINTEN_SPEC, bool, O>;
#[doc = "Field `IEPNEEN` reader - IN endpoint NAK effective interrupt enable"]
pub type IEPNEEN_R = crate::BitReader<bool>;
#[doc = "Field `IEPNEEN` writer - IN endpoint NAK effective interrupt enable"]
pub type IEPNEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINTEN_SPEC, bool, O>;
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
    pub fn tfen(&mut self) -> TFEN_W<0> {
        TFEN_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn epdisen(&mut self) -> EPDISEN_W<1> {
        EPDISEN_W::new(self)
    }
    #[doc = "Bit 3 - Control IN timeout condition interrupt enable (Non-isochronous endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn citoen(&mut self) -> CITOEN_W<3> {
        CITOEN_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint Tx FIFO underrun interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn eptxfuden(&mut self) -> EPTXFUDEN_W<4> {
        EPTXFUDEN_W::new(self)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iepneen(&mut self) -> IEPNEEN_W<6> {
        IEPNEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device IN endpoint common interrupt mask register (DIEPINTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepinten](index.html) module"]
pub struct DIEPINTEN_SPEC;
impl crate::RegisterSpec for DIEPINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepinten::R](R) reader structure"]
impl crate::Readable for DIEPINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepinten::W](W) writer structure"]
impl crate::Writable for DIEPINTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINTEN to value 0"]
impl crate::Resettable for DIEPINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
