#[doc = "Register `TSTAT` reader"]
pub type R = crate::R<TSTAT_SPEC>;
#[doc = "Register `TSTAT` writer"]
pub type W = crate::W<TSTAT_SPEC>;
#[doc = "Field `MTF0` reader - Mailbox 0 transmit finished"]
pub type MTF0_R = crate::BitReader;
#[doc = "Field `MTF0` writer - Mailbox 0 transmit finished"]
pub type MTF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTFNERR0` reader - Mailbox 0 transmit finished and no error"]
pub type MTFNERR0_R = crate::BitReader;
#[doc = "Field `MTFNERR0` writer - Mailbox 0 transmit finished and no error"]
pub type MTFNERR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAL0` reader - Mailbox 0 arbitration lost"]
pub type MAL0_R = crate::BitReader;
#[doc = "Field `MAL0` writer - Mailbox 0 arbitration lost"]
pub type MAL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTE0` reader - Mailbox 0 transmit error"]
pub type MTE0_R = crate::BitReader;
#[doc = "Field `MTE0` writer - Mailbox 0 transmit error"]
pub type MTE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MST0` reader - Mailbox 0 stop transmitting"]
pub type MST0_R = crate::BitReader;
#[doc = "Field `MST0` writer - Mailbox 0 stop transmitting"]
pub type MST0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTF1` reader - Mailbox 1 transmit finished"]
pub type MTF1_R = crate::BitReader;
#[doc = "Field `MTF1` writer - Mailbox 1 transmit finished"]
pub type MTF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTFNERR1` reader - Mailbox 1 transmit finished and no error"]
pub type MTFNERR1_R = crate::BitReader;
#[doc = "Field `MTFNERR1` writer - Mailbox 1 transmit finished and no error"]
pub type MTFNERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAL1` reader - Mailbox 1 arbitration lost"]
pub type MAL1_R = crate::BitReader;
#[doc = "Field `MAL1` writer - Mailbox 1 arbitration lost"]
pub type MAL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTE1` reader - Mailbox 1 transmit error"]
pub type MTE1_R = crate::BitReader;
#[doc = "Field `MTE1` writer - Mailbox 1 transmit error"]
pub type MTE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MST1` reader - Mailbox 1 stop transmitting"]
pub type MST1_R = crate::BitReader;
#[doc = "Field `MST1` writer - Mailbox 1 stop transmitting"]
pub type MST1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTF2` reader - Mailbox 2 transmit finished"]
pub type MTF2_R = crate::BitReader;
#[doc = "Field `MTF2` writer - Mailbox 2 transmit finished"]
pub type MTF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTFNERR2` reader - Mailbox 2 transmit finished and no error"]
pub type MTFNERR2_R = crate::BitReader;
#[doc = "Field `MTFNERR2` writer - Mailbox 2 transmit finished and no error"]
pub type MTFNERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAL2` reader - Mailbox 2 arbitration lost"]
pub type MAL2_R = crate::BitReader;
#[doc = "Field `MAL2` writer - Mailbox 2 arbitration lost"]
pub type MAL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTE2` reader - Mailbox 2 transmit error"]
pub type MTE2_R = crate::BitReader;
#[doc = "Field `MTE2` writer - Mailbox 2 transmit error"]
pub type MTE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MST2` reader - Mailbox 2 stop transmitting"]
pub type MST2_R = crate::BitReader;
#[doc = "Field `MST2` writer - Mailbox 2 stop transmitting"]
pub type MST2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NUM` reader - number of the transmit FIFO mailbox in which the frame will be transmitted if at least one mailbox is empty"]
pub type NUM_R = crate::FieldReader;
#[doc = "Field `TME0` reader - Transmit mailbox 0 empty"]
pub type TME0_R = crate::BitReader;
#[doc = "Field `TME1` reader - Transmit mailbox 1 empty"]
pub type TME1_R = crate::BitReader;
#[doc = "Field `TME2` reader - Transmit mailbox 2 empty"]
pub type TME2_R = crate::BitReader;
#[doc = "Field `TMLS0` reader - Transmit mailbox 0 last sending in transmit FIFO"]
pub type TMLS0_R = crate::BitReader;
#[doc = "Field `TMLS1` reader - Transmit mailbox 1 last sending in transmit FIFO"]
pub type TMLS1_R = crate::BitReader;
#[doc = "Field `TMLS2` reader - Transmit mailbox 2 last sending in transmit FIFO"]
pub type TMLS2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mailbox 0 transmit finished"]
    #[inline(always)]
    pub fn mtf0(&self) -> MTF0_R {
        MTF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox 0 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr0(&self) -> MTFNERR0_R {
        MTFNERR0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox 0 arbitration lost"]
    #[inline(always)]
    pub fn mal0(&self) -> MAL0_R {
        MAL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox 0 transmit error"]
    #[inline(always)]
    pub fn mte0(&self) -> MTE0_R {
        MTE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Mailbox 0 stop transmitting"]
    #[inline(always)]
    pub fn mst0(&self) -> MST0_R {
        MST0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mailbox 1 transmit finished"]
    #[inline(always)]
    pub fn mtf1(&self) -> MTF1_R {
        MTF1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mailbox 1 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr1(&self) -> MTFNERR1_R {
        MTFNERR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mailbox 1 arbitration lost"]
    #[inline(always)]
    pub fn mal1(&self) -> MAL1_R {
        MAL1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mailbox 1 transmit error"]
    #[inline(always)]
    pub fn mte1(&self) -> MTE1_R {
        MTE1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Mailbox 1 stop transmitting"]
    #[inline(always)]
    pub fn mst1(&self) -> MST1_R {
        MST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mailbox 2 transmit finished"]
    #[inline(always)]
    pub fn mtf2(&self) -> MTF2_R {
        MTF2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mailbox 2 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr2(&self) -> MTFNERR2_R {
        MTFNERR2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mailbox 2 arbitration lost"]
    #[inline(always)]
    pub fn mal2(&self) -> MAL2_R {
        MAL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mailbox 2 transmit error"]
    #[inline(always)]
    pub fn mte2(&self) -> MTE2_R {
        MTE2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Mailbox 2 stop transmitting"]
    #[inline(always)]
    pub fn mst2(&self) -> MST2_R {
        MST2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - number of the transmit FIFO mailbox in which the frame will be transmitted if at least one mailbox is empty"]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Transmit mailbox 0 empty"]
    #[inline(always)]
    pub fn tme0(&self) -> TME0_R {
        TME0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit mailbox 1 empty"]
    #[inline(always)]
    pub fn tme1(&self) -> TME1_R {
        TME1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit mailbox 2 empty"]
    #[inline(always)]
    pub fn tme2(&self) -> TME2_R {
        TME2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Transmit mailbox 0 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls0(&self) -> TMLS0_R {
        TMLS0_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit mailbox 1 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls1(&self) -> TMLS1_R {
        TMLS1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmit mailbox 2 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls2(&self) -> TMLS2_R {
        TMLS2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mailbox 0 transmit finished"]
    #[inline(always)]
    #[must_use]
    pub fn mtf0(&mut self) -> MTF0_W<TSTAT_SPEC, 0> {
        MTF0_W::new(self)
    }
    #[doc = "Bit 1 - Mailbox 0 transmit finished and no error"]
    #[inline(always)]
    #[must_use]
    pub fn mtfnerr0(&mut self) -> MTFNERR0_W<TSTAT_SPEC, 1> {
        MTFNERR0_W::new(self)
    }
    #[doc = "Bit 2 - Mailbox 0 arbitration lost"]
    #[inline(always)]
    #[must_use]
    pub fn mal0(&mut self) -> MAL0_W<TSTAT_SPEC, 2> {
        MAL0_W::new(self)
    }
    #[doc = "Bit 3 - Mailbox 0 transmit error"]
    #[inline(always)]
    #[must_use]
    pub fn mte0(&mut self) -> MTE0_W<TSTAT_SPEC, 3> {
        MTE0_W::new(self)
    }
    #[doc = "Bit 7 - Mailbox 0 stop transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn mst0(&mut self) -> MST0_W<TSTAT_SPEC, 7> {
        MST0_W::new(self)
    }
    #[doc = "Bit 8 - Mailbox 1 transmit finished"]
    #[inline(always)]
    #[must_use]
    pub fn mtf1(&mut self) -> MTF1_W<TSTAT_SPEC, 8> {
        MTF1_W::new(self)
    }
    #[doc = "Bit 9 - Mailbox 1 transmit finished and no error"]
    #[inline(always)]
    #[must_use]
    pub fn mtfnerr1(&mut self) -> MTFNERR1_W<TSTAT_SPEC, 9> {
        MTFNERR1_W::new(self)
    }
    #[doc = "Bit 10 - Mailbox 1 arbitration lost"]
    #[inline(always)]
    #[must_use]
    pub fn mal1(&mut self) -> MAL1_W<TSTAT_SPEC, 10> {
        MAL1_W::new(self)
    }
    #[doc = "Bit 11 - Mailbox 1 transmit error"]
    #[inline(always)]
    #[must_use]
    pub fn mte1(&mut self) -> MTE1_W<TSTAT_SPEC, 11> {
        MTE1_W::new(self)
    }
    #[doc = "Bit 15 - Mailbox 1 stop transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn mst1(&mut self) -> MST1_W<TSTAT_SPEC, 15> {
        MST1_W::new(self)
    }
    #[doc = "Bit 16 - Mailbox 2 transmit finished"]
    #[inline(always)]
    #[must_use]
    pub fn mtf2(&mut self) -> MTF2_W<TSTAT_SPEC, 16> {
        MTF2_W::new(self)
    }
    #[doc = "Bit 17 - Mailbox 2 transmit finished and no error"]
    #[inline(always)]
    #[must_use]
    pub fn mtfnerr2(&mut self) -> MTFNERR2_W<TSTAT_SPEC, 17> {
        MTFNERR2_W::new(self)
    }
    #[doc = "Bit 18 - Mailbox 2 arbitration lost"]
    #[inline(always)]
    #[must_use]
    pub fn mal2(&mut self) -> MAL2_W<TSTAT_SPEC, 18> {
        MAL2_W::new(self)
    }
    #[doc = "Bit 19 - Mailbox 2 transmit error"]
    #[inline(always)]
    #[must_use]
    pub fn mte2(&mut self) -> MTE2_W<TSTAT_SPEC, 19> {
        MTE2_W::new(self)
    }
    #[doc = "Bit 23 - Mailbox 2 stop transmitting"]
    #[inline(always)]
    #[must_use]
    pub fn mst2(&mut self) -> MST2_W<TSTAT_SPEC, 23> {
        MST2_W::new(self)
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
#[doc = "Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSTAT_SPEC;
impl crate::RegisterSpec for TSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstat::R`](R) reader structure"]
impl crate::Readable for TSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tstat::W`](W) writer structure"]
impl crate::Writable for TSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSTAT to value 0x1c00_0000"]
impl crate::Resettable for TSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x1c00_0000;
}
