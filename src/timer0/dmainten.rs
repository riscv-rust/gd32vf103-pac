#[doc = "Register `DMAINTEN` reader"]
pub type R = crate::R<DMAINTEN_SPEC>;
#[doc = "Register `DMAINTEN` writer"]
pub type W = crate::W<DMAINTEN_SPEC>;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub type UPIE_R = crate::BitReader;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0IE` reader - Channel 0 capture/compare interrupt enable"]
pub type CH0IE_R = crate::BitReader;
#[doc = "Field `CH0IE` writer - Channel 0 capture/compare interrupt enable"]
pub type CH0IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1IE` reader - Channel 1 capture/compare interrupt enable"]
pub type CH1IE_R = crate::BitReader;
#[doc = "Field `CH1IE` writer - Channel 1 capture/compare interrupt enable"]
pub type CH1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2IE` reader - Channel 2 capture/compare interrupt enable"]
pub type CH2IE_R = crate::BitReader;
#[doc = "Field `CH2IE` writer - Channel 2 capture/compare interrupt enable"]
pub type CH2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3IE` reader - Channel 3 capture/compare interrupt enable"]
pub type CH3IE_R = crate::BitReader;
#[doc = "Field `CH3IE` writer - Channel 3 capture/compare interrupt enable"]
pub type CH3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMTIE` reader - commutation interrupt enable"]
pub type CMTIE_R = crate::BitReader;
#[doc = "Field `CMTIE` writer - commutation interrupt enable"]
pub type CMTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGIE` reader - Trigger interrupt enable"]
pub type TRGIE_R = crate::BitReader;
#[doc = "Field `TRGIE` writer - Trigger interrupt enable"]
pub type TRGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKIE` reader - Break interrupt enable"]
pub type BRKIE_R = crate::BitReader;
#[doc = "Field `BRKIE` writer - Break interrupt enable"]
pub type BRKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UPDEN_R = crate::BitReader;
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UPDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0DEN` reader - Channel 0 capture/compare DMA request enable"]
pub type CH0DEN_R = crate::BitReader;
#[doc = "Field `CH0DEN` writer - Channel 0 capture/compare DMA request enable"]
pub type CH0DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1DEN` reader - Channel 1 capture/compare DMA request enable"]
pub type CH1DEN_R = crate::BitReader;
#[doc = "Field `CH1DEN` writer - Channel 1 capture/compare DMA request enable"]
pub type CH1DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2DEN` reader - Channel 2 capture/compare DMA request enable"]
pub type CH2DEN_R = crate::BitReader;
#[doc = "Field `CH2DEN` writer - Channel 2 capture/compare DMA request enable"]
pub type CH2DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3DEN` reader - Channel 3 capture/compare DMA request enable"]
pub type CH3DEN_R = crate::BitReader;
#[doc = "Field `CH3DEN` writer - Channel 3 capture/compare DMA request enable"]
pub type CH3DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMTDEN` reader - Commutation DMA request enable"]
pub type CMTDEN_R = crate::BitReader;
#[doc = "Field `CMTDEN` writer - Commutation DMA request enable"]
pub type CMTDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGDEN` reader - Trigger DMA request enable"]
pub type TRGDEN_R = crate::BitReader;
#[doc = "Field `TRGDEN` writer - Trigger DMA request enable"]
pub type TRGDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&self) -> CH0IE_R {
        CH0IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch1ie(&self) -> CH1IE_R {
        CH1IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch2ie(&self) -> CH2IE_R {
        CH2IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch3ie(&self) -> CH3IE_R {
        CH3IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - commutation interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&self) -> CMTIE_R {
        CMTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&self) -> TRGIE_R {
        TRGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn brkie(&self) -> BRKIE_R {
        BRKIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UPDEN_R {
        UPDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&self) -> CH0DEN_R {
        CH0DEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 1 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch1den(&self) -> CH1DEN_R {
        CH1DEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch2den(&self) -> CH2DEN_R {
        CH2DEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch3den(&self) -> CH3DEN_R {
        CH3DEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Commutation DMA request enable"]
    #[inline(always)]
    pub fn cmtden(&self) -> CMTDEN_R {
        CMTDEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn trgden(&self) -> TRGDEN_R {
        TRGDEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<DMAINTEN_SPEC> {
        UPIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ie(&mut self) -> CH0IE_W<DMAINTEN_SPEC> {
        CH0IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ie(&mut self) -> CH1IE_W<DMAINTEN_SPEC> {
        CH1IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ie(&mut self) -> CH2IE_W<DMAINTEN_SPEC> {
        CH2IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ie(&mut self) -> CH3IE_W<DMAINTEN_SPEC> {
        CH3IE_W::new(self, 4)
    }
    #[doc = "Bit 5 - commutation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtie(&mut self) -> CMTIE_W<DMAINTEN_SPEC> {
        CMTIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgie(&mut self) -> TRGIE_W<DMAINTEN_SPEC> {
        TRGIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn brkie(&mut self) -> BRKIE_W<DMAINTEN_SPEC> {
        BRKIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn upden(&mut self) -> UPDEN_W<DMAINTEN_SPEC> {
        UPDEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 0 capture/compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0den(&mut self) -> CH0DEN_W<DMAINTEN_SPEC> {
        CH0DEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 1 capture/compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1den(&mut self) -> CH1DEN_W<DMAINTEN_SPEC> {
        CH1DEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 2 capture/compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2den(&mut self) -> CH2DEN_W<DMAINTEN_SPEC> {
        CH2DEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3den(&mut self) -> CH3DEN_W<DMAINTEN_SPEC> {
        CH3DEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Commutation DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtden(&mut self) -> CMTDEN_W<DMAINTEN_SPEC> {
        CMTDEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgden(&mut self) -> TRGDEN_W<DMAINTEN_SPEC> {
        TRGDEN_W::new(self, 14)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmainten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmainten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAINTEN_SPEC;
impl crate::RegisterSpec for DMAINTEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmainten::R`](R) reader structure"]
impl crate::Readable for DMAINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmainten::W`](W) writer structure"]
impl crate::Writable for DMAINTEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DMAINTEN_SPEC {
    const RESET_VALUE: u16 = 0;
}
