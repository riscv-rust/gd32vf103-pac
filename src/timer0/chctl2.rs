#[doc = "Register `CHCTL2` reader"]
pub type R = crate::R<CHCTL2_SPEC>;
#[doc = "Register `CHCTL2` writer"]
pub type W = crate::W<CHCTL2_SPEC>;
#[doc = "Field `CH0EN` reader - Channel 0 capture/compare function enable"]
pub type CH0EN_R = crate::BitReader;
#[doc = "Field `CH0EN` writer - Channel 0 capture/compare function enable"]
pub type CH0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0P` reader - Channel 0 capture/compare function polarity"]
pub type CH0P_R = crate::BitReader;
#[doc = "Field `CH0P` writer - Channel 0 capture/compare function polarity"]
pub type CH0P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0NEN` reader - Channel 0 complementary output enable"]
pub type CH0NEN_R = crate::BitReader;
#[doc = "Field `CH0NEN` writer - Channel 0 complementary output enable"]
pub type CH0NEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0NP` reader - Channel 0 complementary output polarity"]
pub type CH0NP_R = crate::BitReader;
#[doc = "Field `CH0NP` writer - Channel 0 complementary output polarity"]
pub type CH0NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1EN` reader - Channel 1 capture/compare function enable"]
pub type CH1EN_R = crate::BitReader;
#[doc = "Field `CH1EN` writer - Channel 1 capture/compare function enable"]
pub type CH1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1P` reader - Channel 1 capture/compare function polarity"]
pub type CH1P_R = crate::BitReader;
#[doc = "Field `CH1P` writer - Channel 1 capture/compare function polarity"]
pub type CH1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1NEN` reader - Channel 1 complementary output enable"]
pub type CH1NEN_R = crate::BitReader;
#[doc = "Field `CH1NEN` writer - Channel 1 complementary output enable"]
pub type CH1NEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1NP` reader - Channel 1 complementary output polarity"]
pub type CH1NP_R = crate::BitReader;
#[doc = "Field `CH1NP` writer - Channel 1 complementary output polarity"]
pub type CH1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2EN` reader - Channel 2 capture/compare function enable"]
pub type CH2EN_R = crate::BitReader;
#[doc = "Field `CH2EN` writer - Channel 2 capture/compare function enable"]
pub type CH2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2P` reader - Channel 2 capture/compare function polarity"]
pub type CH2P_R = crate::BitReader;
#[doc = "Field `CH2P` writer - Channel 2 capture/compare function polarity"]
pub type CH2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2NEN` reader - Channel 2 complementary output enable"]
pub type CH2NEN_R = crate::BitReader;
#[doc = "Field `CH2NEN` writer - Channel 2 complementary output enable"]
pub type CH2NEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2NP` reader - Channel 2 complementary output polarity"]
pub type CH2NP_R = crate::BitReader;
#[doc = "Field `CH2NP` writer - Channel 2 complementary output polarity"]
pub type CH2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3EN` reader - Channel 3 capture/compare function enable"]
pub type CH3EN_R = crate::BitReader;
#[doc = "Field `CH3EN` writer - Channel 3 capture/compare function enable"]
pub type CH3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3P` reader - Channel 3 capture/compare function polarity"]
pub type CH3P_R = crate::BitReader;
#[doc = "Field `CH3P` writer - Channel 3 capture/compare function polarity"]
pub type CH3P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 0 capture/compare function enable"]
    #[inline(always)]
    pub fn ch0en(&self) -> CH0EN_R {
        CH0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch0p(&self) -> CH0P_R {
        CH0P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 complementary output enable"]
    #[inline(always)]
    pub fn ch0nen(&self) -> CH0NEN_R {
        CH0NEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    pub fn ch0np(&self) -> CH0NP_R {
        CH0NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 capture/compare function enable"]
    #[inline(always)]
    pub fn ch1en(&self) -> CH1EN_R {
        CH1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch1p(&self) -> CH1P_R {
        CH1P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 1 complementary output enable"]
    #[inline(always)]
    pub fn ch1nen(&self) -> CH1NEN_R {
        CH1NEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 1 complementary output polarity"]
    #[inline(always)]
    pub fn ch1np(&self) -> CH1NP_R {
        CH1NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 2 capture/compare function enable"]
    #[inline(always)]
    pub fn ch2en(&self) -> CH2EN_R {
        CH2EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 2 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch2p(&self) -> CH2P_R {
        CH2P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 complementary output enable"]
    #[inline(always)]
    pub fn ch2nen(&self) -> CH2NEN_R {
        CH2NEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 complementary output polarity"]
    #[inline(always)]
    pub fn ch2np(&self) -> CH2NP_R {
        CH2NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare function enable"]
    #[inline(always)]
    pub fn ch3en(&self) -> CH3EN_R {
        CH3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 3 capture/compare function polarity"]
    #[inline(always)]
    pub fn ch3p(&self) -> CH3P_R {
        CH3P_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 capture/compare function enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0en(&mut self) -> CH0EN_W<CHCTL2_SPEC> {
        CH0EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare function polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0p(&mut self) -> CH0P_W<CHCTL2_SPEC> {
        CH0P_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 0 complementary output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0nen(&mut self) -> CH0NEN_W<CHCTL2_SPEC> {
        CH0NEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 0 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0np(&mut self) -> CH0NP_W<CHCTL2_SPEC> {
        CH0NP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 1 capture/compare function enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1en(&mut self) -> CH1EN_W<CHCTL2_SPEC> {
        CH1EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 1 capture/compare function polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch1p(&mut self) -> CH1P_W<CHCTL2_SPEC> {
        CH1P_W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 1 complementary output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1nen(&mut self) -> CH1NEN_W<CHCTL2_SPEC> {
        CH1NEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 1 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch1np(&mut self) -> CH1NP_W<CHCTL2_SPEC> {
        CH1NP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 2 capture/compare function enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2en(&mut self) -> CH2EN_W<CHCTL2_SPEC> {
        CH2EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 2 capture/compare function polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch2p(&mut self) -> CH2P_W<CHCTL2_SPEC> {
        CH2P_W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 complementary output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2nen(&mut self) -> CH2NEN_W<CHCTL2_SPEC> {
        CH2NEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 2 complementary output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch2np(&mut self) -> CH2NP_W<CHCTL2_SPEC> {
        CH2NP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare function enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3en(&mut self) -> CH3EN_W<CHCTL2_SPEC> {
        CH3EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 3 capture/compare function polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch3p(&mut self) -> CH3P_W<CHCTL2_SPEC> {
        CH3P_W::new(self, 13)
    }
}
#[doc = "Channel control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`chctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTL2_SPEC;
impl crate::RegisterSpec for CHCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chctl2::R`](R) reader structure"]
impl crate::Readable for CHCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctl2::W`](W) writer structure"]
impl crate::Writable for CHCTL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CHCTL2 to value 0"]
impl crate::Resettable for CHCTL2_SPEC {
    const RESET_VALUE: u16 = 0;
}
