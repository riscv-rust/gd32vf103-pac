#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `TMEIE` reader - Transmit mailbox empty interrupt enable"]
pub type TMEIE_R = crate::BitReader;
#[doc = "Field `TMEIE` writer - Transmit mailbox empty interrupt enable"]
pub type TMEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFNEIE0` reader - Receive FIFO0 not empty interrupt enable"]
pub type RFNEIE0_R = crate::BitReader;
#[doc = "Field `RFNEIE0` writer - Receive FIFO0 not empty interrupt enable"]
pub type RFNEIE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFFIE0` reader - Receive FIFO0 full interrupt enable"]
pub type RFFIE0_R = crate::BitReader;
#[doc = "Field `RFFIE0` writer - Receive FIFO0 full interrupt enable"]
pub type RFFIE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFOIE0` reader - Receive FIFO0 overfull interrupt enable"]
pub type RFOIE0_R = crate::BitReader;
#[doc = "Field `RFOIE0` writer - Receive FIFO0 overfull interrupt enable"]
pub type RFOIE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFNEIE1` reader - Receive FIFO1 not empty interrupt enable"]
pub type RFNEIE1_R = crate::BitReader;
#[doc = "Field `RFNEIE1` writer - Receive FIFO1 not empty interrupt enable"]
pub type RFNEIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFFIE1` reader - Receive FIFO1 full interrupt enable"]
pub type RFFIE1_R = crate::BitReader;
#[doc = "Field `RFFIE1` writer - Receive FIFO1 full interrupt enable"]
pub type RFFIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFOIE1` reader - Receive FIFO1 overfull interrupt enable"]
pub type RFOIE1_R = crate::BitReader;
#[doc = "Field `RFOIE1` writer - Receive FIFO1 overfull interrupt enable"]
pub type RFOIE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WERRIE` reader - Warning error interrupt enable"]
pub type WERRIE_R = crate::BitReader;
#[doc = "Field `WERRIE` writer - Warning error interrupt enable"]
pub type WERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERRIE` reader - Passive error interrupt enable"]
pub type PERRIE_R = crate::BitReader;
#[doc = "Field `PERRIE` writer - Passive error interrupt enable"]
pub type PERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOIE` reader - Bus-off interrupt enable"]
pub type BOIE_R = crate::BitReader;
#[doc = "Field `BOIE` writer - Bus-off interrupt enable"]
pub type BOIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRNIE` reader - Error number interrupt enable"]
pub type ERRNIE_R = crate::BitReader;
#[doc = "Field `ERRNIE` writer - Error number interrupt enable"]
pub type ERRNIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WIE` reader - Wakeup interrupt enable"]
pub type WIE_R = crate::BitReader;
#[doc = "Field `WIE` writer - Wakeup interrupt enable"]
pub type WIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLPWIE` reader - Sleep working interrupt enable"]
pub type SLPWIE_R = crate::BitReader;
#[doc = "Field `SLPWIE` writer - Sleep working interrupt enable"]
pub type SLPWIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Transmit mailbox empty interrupt enable"]
    #[inline(always)]
    pub fn tmeie(&self) -> TMEIE_R {
        TMEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO0 not empty interrupt enable"]
    #[inline(always)]
    pub fn rfneie0(&self) -> RFNEIE0_R {
        RFNEIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO0 full interrupt enable"]
    #[inline(always)]
    pub fn rffie0(&self) -> RFFIE0_R {
        RFFIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO0 overfull interrupt enable"]
    #[inline(always)]
    pub fn rfoie0(&self) -> RFOIE0_R {
        RFOIE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO1 not empty interrupt enable"]
    #[inline(always)]
    pub fn rfneie1(&self) -> RFNEIE1_R {
        RFNEIE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO1 full interrupt enable"]
    #[inline(always)]
    pub fn rffie1(&self) -> RFFIE1_R {
        RFFIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO1 overfull interrupt enable"]
    #[inline(always)]
    pub fn rfoie1(&self) -> RFOIE1_R {
        RFOIE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Warning error interrupt enable"]
    #[inline(always)]
    pub fn werrie(&self) -> WERRIE_R {
        WERRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Passive error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    pub fn boie(&self) -> BOIE_R {
        BOIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error number interrupt enable"]
    #[inline(always)]
    pub fn errnie(&self) -> ERRNIE_R {
        ERRNIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Wakeup interrupt enable"]
    #[inline(always)]
    pub fn wie(&self) -> WIE_R {
        WIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sleep working interrupt enable"]
    #[inline(always)]
    pub fn slpwie(&self) -> SLPWIE_R {
        SLPWIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit mailbox empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmeie(&mut self) -> TMEIE_W<INTEN_SPEC, 0> {
        TMEIE_W::new(self)
    }
    #[doc = "Bit 1 - Receive FIFO0 not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfneie0(&mut self) -> RFNEIE0_W<INTEN_SPEC, 1> {
        RFNEIE0_W::new(self)
    }
    #[doc = "Bit 2 - Receive FIFO0 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rffie0(&mut self) -> RFFIE0_W<INTEN_SPEC, 2> {
        RFFIE0_W::new(self)
    }
    #[doc = "Bit 3 - Receive FIFO0 overfull interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfoie0(&mut self) -> RFOIE0_W<INTEN_SPEC, 3> {
        RFOIE0_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO1 not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfneie1(&mut self) -> RFNEIE1_W<INTEN_SPEC, 4> {
        RFNEIE1_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO1 full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rffie1(&mut self) -> RFFIE1_W<INTEN_SPEC, 5> {
        RFFIE1_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO1 overfull interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfoie1(&mut self) -> RFOIE1_W<INTEN_SPEC, 6> {
        RFOIE1_W::new(self)
    }
    #[doc = "Bit 8 - Warning error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn werrie(&mut self) -> WERRIE_W<INTEN_SPEC, 8> {
        WERRIE_W::new(self)
    }
    #[doc = "Bit 9 - Passive error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn perrie(&mut self) -> PERRIE_W<INTEN_SPEC, 9> {
        PERRIE_W::new(self)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn boie(&mut self) -> BOIE_W<INTEN_SPEC, 10> {
        BOIE_W::new(self)
    }
    #[doc = "Bit 11 - Error number interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errnie(&mut self) -> ERRNIE_W<INTEN_SPEC, 11> {
        ERRNIE_W::new(self)
    }
    #[doc = "Bit 15 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<INTEN_SPEC, 15> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 16 - Wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wie(&mut self) -> WIE_W<INTEN_SPEC, 16> {
        WIE_W::new(self)
    }
    #[doc = "Bit 17 - Sleep working interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn slpwie(&mut self) -> SLPWIE_W<INTEN_SPEC, 17> {
        SLPWIE_W::new(self)
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
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
