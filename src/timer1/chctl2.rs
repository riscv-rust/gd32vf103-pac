#[doc = "Register `CHCTL2` reader"]
pub type R = crate::R<CHCTL2_SPEC>;
#[doc = "Register `CHCTL2` writer"]
pub type W = crate::W<CHCTL2_SPEC>;
#[doc = "Field `CH0EN` reader - Channel 0 capture/compare function enable"]
pub type CH0EN_R = crate::BitReader;
#[doc = "Field `CH0EN` writer - Channel 0 capture/compare function enable"]
pub type CH0EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH0P` reader - Channel 0 capture/compare function polarity"]
pub type CH0P_R = crate::BitReader;
#[doc = "Field `CH0P` writer - Channel 0 capture/compare function polarity"]
pub type CH0P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1EN` reader - Channel 1 capture/compare function enable"]
pub type CH1EN_R = crate::BitReader;
#[doc = "Field `CH1EN` writer - Channel 1 capture/compare function enable"]
pub type CH1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH1P` reader - Channel 1 capture/compare function polarity"]
pub type CH1P_R = crate::BitReader;
#[doc = "Field `CH1P` writer - Channel 1 capture/compare function polarity"]
pub type CH1P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2EN` reader - Channel 2 capture/compare function enable"]
pub type CH2EN_R = crate::BitReader;
#[doc = "Field `CH2EN` writer - Channel 2 capture/compare function enable"]
pub type CH2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH2P` reader - Channel 2 capture/compare function polarity"]
pub type CH2P_R = crate::BitReader;
#[doc = "Field `CH2P` writer - Channel 2 capture/compare function polarity"]
pub type CH2P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3EN` reader - Channel 3 capture/compare function enable"]
pub type CH3EN_R = crate::BitReader;
#[doc = "Field `CH3EN` writer - Channel 3 capture/compare function enable"]
pub type CH3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CH3P` reader - Channel 3 capture/compare function polarity"]
pub type CH3P_R = crate::BitReader;
#[doc = "Field `CH3P` writer - Channel 3 capture/compare function polarity"]
pub type CH3P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn ch0en(&mut self) -> CH0EN_W<CHCTL2_SPEC, 0> {
        CH0EN_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare function polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch0p(&mut self) -> CH0P_W<CHCTL2_SPEC, 1> {
        CH0P_W::new(self)
    }
    #[doc = "Bit 4 - Channel 1 capture/compare function enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1en(&mut self) -> CH1EN_W<CHCTL2_SPEC, 4> {
        CH1EN_W::new(self)
    }
    #[doc = "Bit 5 - Channel 1 capture/compare function polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch1p(&mut self) -> CH1P_W<CHCTL2_SPEC, 5> {
        CH1P_W::new(self)
    }
    #[doc = "Bit 8 - Channel 2 capture/compare function enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2en(&mut self) -> CH2EN_W<CHCTL2_SPEC, 8> {
        CH2EN_W::new(self)
    }
    #[doc = "Bit 9 - Channel 2 capture/compare function polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch2p(&mut self) -> CH2P_W<CHCTL2_SPEC, 9> {
        CH2P_W::new(self)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare function enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3en(&mut self) -> CH3EN_W<CHCTL2_SPEC, 12> {
        CH3EN_W::new(self)
    }
    #[doc = "Bit 13 - Channel 3 capture/compare function polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ch3p(&mut self) -> CH3P_W<CHCTL2_SPEC, 13> {
        CH3P_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCTL2_SPEC;
impl crate::RegisterSpec for CHCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chctl2::R`](R) reader structure"]
impl crate::Readable for CHCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chctl2::W`](W) writer structure"]
impl crate::Writable for CHCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTL2 to value 0"]
impl crate::Resettable for CHCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
