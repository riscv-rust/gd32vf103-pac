#[doc = "Register `DMAINTEN` reader"]
pub struct R(crate::R<DMAINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAINTEN` writer"]
pub struct W(crate::W<DMAINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAINTEN_SPEC>;
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
impl From<crate::W<DMAINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub type UPIE_R = crate::BitReader<bool>;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UPIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
#[doc = "Field `CH0IE` reader - Channel 0 capture/compare interrupt enable"]
pub type CH0IE_R = crate::BitReader<bool>;
#[doc = "Field `CH0IE` writer - Channel 0 capture/compare interrupt enable"]
pub type CH0IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
#[doc = "Field `CH1IE` reader - Channel 1 capture/compare interrupt enable"]
pub type CH1IE_R = crate::BitReader<bool>;
#[doc = "Field `CH1IE` writer - Channel 1 capture/compare interrupt enable"]
pub type CH1IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
#[doc = "Field `CH2IE` reader - Channel 2 capture/compare interrupt enable"]
pub type CH2IE_R = crate::BitReader<bool>;
#[doc = "Field `CH2IE` writer - Channel 2 capture/compare interrupt enable"]
pub type CH2IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
#[doc = "Field `CH3IE` reader - Channel 3 capture/compare interrupt enable"]
pub type CH3IE_R = crate::BitReader<bool>;
#[doc = "Field `CH3IE` writer - Channel 3 capture/compare interrupt enable"]
pub type CH3IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
#[doc = "Field `TRGIE` reader - Trigger interrupt enable"]
pub type TRGIE_R = crate::BitReader<bool>;
#[doc = "Field `TRGIE` writer - Trigger interrupt enable"]
pub type TRGIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UPDEN_R = crate::BitReader<bool>;
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UPDEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
#[doc = "Field `CH0DEN` reader - Channel 0 capture/compare DMA request enable"]
pub type CH0DEN_R = crate::BitReader<bool>;
#[doc = "Field `CH0DEN` writer - Channel 0 capture/compare DMA request enable"]
pub type CH0DEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
#[doc = "Field `CH1DEN` reader - Channel 1 capture/compare DMA request enable"]
pub type CH1DEN_R = crate::BitReader<bool>;
#[doc = "Field `CH1DEN` writer - Channel 1 capture/compare DMA request enable"]
pub type CH1DEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
#[doc = "Field `CH2DEN` reader - Channel 2 capture/compare DMA request enable"]
pub type CH2DEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2DEN` writer - Channel 2 capture/compare DMA request enable"]
pub type CH2DEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
#[doc = "Field `CH3DEN` reader - Channel 3 capture/compare DMA request enable"]
pub type CH3DEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3DEN` writer - Channel 3 capture/compare DMA request enable"]
pub type CH3DEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
#[doc = "Field `TRGDEN` reader - Trigger DMA request enable"]
pub type TRGDEN_R = crate::BitReader<bool>;
#[doc = "Field `TRGDEN` writer - Trigger DMA request enable"]
pub type TRGDEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMAINTEN_SPEC, bool, O>;
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
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn trgie(&self) -> TRGIE_R {
        TRGIE_R::new(((self.bits >> 6) & 1) != 0)
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
    pub fn upie(&mut self) -> UPIE_W<0> {
        UPIE_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ie(&mut self) -> CH0IE_W<1> {
        CH0IE_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1ie(&mut self) -> CH1IE_W<2> {
        CH1IE_W::new(self)
    }
    #[doc = "Bit 3 - Channel 2 capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ie(&mut self) -> CH2IE_W<3> {
        CH2IE_W::new(self)
    }
    #[doc = "Bit 4 - Channel 3 capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ie(&mut self) -> CH3IE_W<4> {
        CH3IE_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgie(&mut self) -> TRGIE_W<6> {
        TRGIE_W::new(self)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn upden(&mut self) -> UPDEN_W<8> {
        UPDEN_W::new(self)
    }
    #[doc = "Bit 9 - Channel 0 capture/compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0den(&mut self) -> CH0DEN_W<9> {
        CH0DEN_W::new(self)
    }
    #[doc = "Bit 10 - Channel 1 capture/compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1den(&mut self) -> CH1DEN_W<10> {
        CH1DEN_W::new(self)
    }
    #[doc = "Bit 11 - Channel 2 capture/compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2den(&mut self) -> CH2DEN_W<11> {
        CH2DEN_W::new(self)
    }
    #[doc = "Bit 12 - Channel 3 capture/compare DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3den(&mut self) -> CH3DEN_W<12> {
        CH3DEN_W::new(self)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgden(&mut self) -> TRGDEN_W<14> {
        TRGDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA/Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmainten](index.html) module"]
pub struct DMAINTEN_SPEC;
impl crate::RegisterSpec for DMAINTEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmainten::R](R) reader structure"]
impl crate::Readable for DMAINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmainten::W](W) writer structure"]
impl crate::Writable for DMAINTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DMAINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
