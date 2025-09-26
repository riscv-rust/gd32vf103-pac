#[doc = "Register `HCH1INTEN` reader"]
pub type R = crate::R<HCH1INTEN_SPEC>;
#[doc = "Register `HCH1INTEN` writer"]
pub type W = crate::W<HCH1INTEN_SPEC>;
#[doc = "Field `TFIE` reader - Transfer completed interrupt enable"]
pub type TFIE_R = crate::BitReader;
#[doc = "Field `TFIE` writer - Transfer completed interrupt enable"]
pub type TFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHIE` reader - Channel halted interrupt enable"]
pub type CHIE_R = crate::BitReader;
#[doc = "Field `CHIE` writer - Channel halted interrupt enable"]
pub type CHIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STALLIE` reader - STALL interrupt enable"]
pub type STALLIE_R = crate::BitReader;
#[doc = "Field `STALLIE` writer - STALL interrupt enable"]
pub type STALLIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKIE` reader - NAK interrupt enable"]
pub type NAKIE_R = crate::BitReader;
#[doc = "Field `NAKIE` writer - NAK interrupt enable"]
pub type NAKIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACKIE` reader - ACK interrupt enable"]
pub type ACKIE_R = crate::BitReader;
#[doc = "Field `ACKIE` writer - ACK interrupt enable"]
pub type ACKIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USBERIE` reader - USB bus error interrupt enable"]
pub type USBERIE_R = crate::BitReader;
#[doc = "Field `USBERIE` writer - USB bus error interrupt enable"]
pub type USBERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BBERIE` reader - Babble error interrupt enable"]
pub type BBERIE_R = crate::BitReader;
#[doc = "Field `BBERIE` writer - Babble error interrupt enable"]
pub type BBERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REQOVRIE` reader - request queue overrun interrupt enable"]
pub type REQOVRIE_R = crate::BitReader;
#[doc = "Field `REQOVRIE` writer - request queue overrun interrupt enable"]
pub type REQOVRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTERIE` reader - Data toggle error interrupt enable"]
pub type DTERIE_R = crate::BitReader;
#[doc = "Field `DTERIE` writer - Data toggle error interrupt enable"]
pub type DTERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn tfie(&mut self) -> TFIE_W<HCH1INTEN_SPEC, 0> {
        TFIE_W::new(self)
    }
    #[doc = "Bit 1 - Channel halted interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn chie(&mut self) -> CHIE_W<HCH1INTEN_SPEC, 1> {
        CHIE_W::new(self)
    }
    #[doc = "Bit 3 - STALL interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallie(&mut self) -> STALLIE_W<HCH1INTEN_SPEC, 3> {
        STALLIE_W::new(self)
    }
    #[doc = "Bit 4 - NAK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nakie(&mut self) -> NAKIE_W<HCH1INTEN_SPEC, 4> {
        NAKIE_W::new(self)
    }
    #[doc = "Bit 5 - ACK interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ackie(&mut self) -> ACKIE_W<HCH1INTEN_SPEC, 5> {
        ACKIE_W::new(self)
    }
    #[doc = "Bit 7 - USB bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn usberie(&mut self) -> USBERIE_W<HCH1INTEN_SPEC, 7> {
        USBERIE_W::new(self)
    }
    #[doc = "Bit 8 - Babble error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bberie(&mut self) -> BBERIE_W<HCH1INTEN_SPEC, 8> {
        BBERIE_W::new(self)
    }
    #[doc = "Bit 9 - request queue overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn reqovrie(&mut self) -> REQOVRIE_W<HCH1INTEN_SPEC, 9> {
        REQOVRIE_W::new(self)
    }
    #[doc = "Bit 10 - Data toggle error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dterie(&mut self) -> DTERIE_W<HCH1INTEN_SPEC, 10> {
        DTERIE_W::new(self)
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
#[doc = "host channel-1 interrupt enable register (HCH1INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch1inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch1inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCH1INTEN_SPEC;
impl crate::RegisterSpec for HCH1INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch1inten::R`](R) reader structure"]
impl crate::Readable for HCH1INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hch1inten::W`](W) writer structure"]
impl crate::Writable for HCH1INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCH1INTEN to value 0"]
impl crate::Resettable for HCH1INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
