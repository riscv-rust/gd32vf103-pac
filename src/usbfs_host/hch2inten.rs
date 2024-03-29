#[doc = "Register `HCH2INTEN` reader"]
pub struct R(crate::R<HCH2INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCH2INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCH2INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCH2INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCH2INTEN` writer"]
pub struct W(crate::W<HCH2INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCH2INTEN_SPEC>;
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
impl From<crate::W<HCH2INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCH2INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFIE` reader - Transfer completed interrupt enable"]
pub type TFIE_R = crate::BitReader<bool>;
#[doc = "Field `TFIE` writer - Transfer completed interrupt enable"]
pub type TFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH2INTEN_SPEC, bool, O>;
#[doc = "Field `CHIE` reader - Channel halted interrupt enable"]
pub type CHIE_R = crate::BitReader<bool>;
#[doc = "Field `CHIE` writer - Channel halted interrupt enable"]
pub type CHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH2INTEN_SPEC, bool, O>;
#[doc = "Field `STALLIE` reader - STALL interrupt enable"]
pub type STALLIE_R = crate::BitReader<bool>;
#[doc = "Field `STALLIE` writer - STALL interrupt enable"]
pub type STALLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH2INTEN_SPEC, bool, O>;
#[doc = "Field `NAKIE` reader - NAK interrupt enable"]
pub type NAKIE_R = crate::BitReader<bool>;
#[doc = "Field `NAKIE` writer - NAK interrupt enable"]
pub type NAKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH2INTEN_SPEC, bool, O>;
#[doc = "Field `ACKIE` reader - ACK interrupt enable"]
pub type ACKIE_R = crate::BitReader<bool>;
#[doc = "Field `ACKIE` writer - ACK interrupt enable"]
pub type ACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH2INTEN_SPEC, bool, O>;
#[doc = "Field `USBERIE` reader - USB bus error interrupt enable"]
pub type USBERIE_R = crate::BitReader<bool>;
#[doc = "Field `USBERIE` writer - USB bus error interrupt enable"]
pub type USBERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH2INTEN_SPEC, bool, O>;
#[doc = "Field `BBERIE` reader - Babble error interrupt enable"]
pub type BBERIE_R = crate::BitReader<bool>;
#[doc = "Field `BBERIE` writer - Babble error interrupt enable"]
pub type BBERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH2INTEN_SPEC, bool, O>;
#[doc = "Field `REQOVRIE` reader - request queue overrun interrupt enable"]
pub type REQOVRIE_R = crate::BitReader<bool>;
#[doc = "Field `REQOVRIE` writer - request queue overrun interrupt enable"]
pub type REQOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH2INTEN_SPEC, bool, O>;
#[doc = "Field `DTERIE` reader - Data toggle error interrupt enable"]
pub type DTERIE_R = crate::BitReader<bool>;
#[doc = "Field `DTERIE` writer - Data toggle error interrupt enable"]
pub type DTERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH2INTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt enable"]
    #[inline(always)]
    pub fn tfie(&self) -> TFIE_R {
        TFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted interrupt enable"]
    #[inline(always)]
    pub fn chie(&self) -> CHIE_R {
        CHIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL interrupt enable"]
    #[inline(always)]
    pub fn stallie(&self) -> STALLIE_R {
        STALLIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK interrupt enable"]
    #[inline(always)]
    pub fn nakie(&self) -> NAKIE_R {
        NAKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK interrupt enable"]
    #[inline(always)]
    pub fn ackie(&self) -> ACKIE_R {
        ACKIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USB bus error interrupt enable"]
    #[inline(always)]
    pub fn usberie(&self) -> USBERIE_R {
        USBERIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error interrupt enable"]
    #[inline(always)]
    pub fn bberie(&self) -> BBERIE_R {
        BBERIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - request queue overrun interrupt enable"]
    #[inline(always)]
    pub fn reqovrie(&self) -> REQOVRIE_R {
        REQOVRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error interrupt enable"]
    #[inline(always)]
    pub fn dterie(&self) -> DTERIE_R {
        DTERIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfie(&mut self) -> TFIE_W<0> {
        TFIE_W::new(self)
    }
    #[doc = "Bit 1 - Channel halted interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn chie(&mut self) -> CHIE_W<1> {
        CHIE_W::new(self)
    }
    #[doc = "Bit 3 - STALL interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallie(&mut self) -> STALLIE_W<3> {
        STALLIE_W::new(self)
    }
    #[doc = "Bit 4 - NAK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakie(&mut self) -> NAKIE_W<4> {
        NAKIE_W::new(self)
    }
    #[doc = "Bit 5 - ACK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackie(&mut self) -> ACKIE_W<5> {
        ACKIE_W::new(self)
    }
    #[doc = "Bit 7 - USB bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn usberie(&mut self) -> USBERIE_W<7> {
        USBERIE_W::new(self)
    }
    #[doc = "Bit 8 - Babble error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bberie(&mut self) -> BBERIE_W<8> {
        BBERIE_W::new(self)
    }
    #[doc = "Bit 9 - request queue overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn reqovrie(&mut self) -> REQOVRIE_W<9> {
        REQOVRIE_W::new(self)
    }
    #[doc = "Bit 10 - Data toggle error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dterie(&mut self) -> DTERIE_W<10> {
        DTERIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "host channel-2 interrupt enable register (HCH2INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch2inten](index.html) module"]
pub struct HCH2INTEN_SPEC;
impl crate::RegisterSpec for HCH2INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hch2inten::R](R) reader structure"]
impl crate::Readable for HCH2INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hch2inten::W](W) writer structure"]
impl crate::Writable for HCH2INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCH2INTEN to value 0"]
impl crate::Resettable for HCH2INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
