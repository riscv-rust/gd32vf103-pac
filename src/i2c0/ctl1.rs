#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CCLK` reader - I2C Peripheral clock frequency"]
pub type I2CCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2CCLK` writer - I2C Peripheral clock frequency"]
pub type I2CCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CTL1_SPEC, u8, u8, 6, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL1_SPEC, bool, O>;
#[doc = "Field `EVIE` reader - Event interrupt enable"]
pub type EVIE_R = crate::BitReader<bool>;
#[doc = "Field `EVIE` writer - Event interrupt enable"]
pub type EVIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL1_SPEC, bool, O>;
#[doc = "Field `BUFIE` reader - Buffer interrupt enable"]
pub type BUFIE_R = crate::BitReader<bool>;
#[doc = "Field `BUFIE` writer - Buffer interrupt enable"]
pub type BUFIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL1_SPEC, bool, O>;
#[doc = "Field `DMAON` reader - DMA mode switch"]
pub type DMAON_R = crate::BitReader<bool>;
#[doc = "Field `DMAON` writer - DMA mode switch"]
pub type DMAON_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL1_SPEC, bool, O>;
#[doc = "Field `DMALST` reader - Flag indicating DMA last transfer"]
pub type DMALST_R = crate::BitReader<bool>;
#[doc = "Field `DMALST` writer - Flag indicating DMA last transfer"]
pub type DMALST_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - I2C Peripheral clock frequency"]
    #[inline(always)]
    pub fn i2cclk(&self) -> I2CCLK_R {
        I2CCLK_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evie(&self) -> EVIE_R {
        EVIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn bufie(&self) -> BUFIE_R {
        BUFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA mode switch"]
    #[inline(always)]
    pub fn dmaon(&self) -> DMAON_R {
        DMAON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Flag indicating DMA last transfer"]
    #[inline(always)]
    pub fn dmalst(&self) -> DMALST_R {
        DMALST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2C Peripheral clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn i2cclk(&mut self) -> I2CCLK_W<0> {
        I2CCLK_W::new(self)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<8> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn evie(&mut self) -> EVIE_W<9> {
        EVIE_W::new(self)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufie(&mut self) -> BUFIE_W<10> {
        BUFIE_W::new(self)
    }
    #[doc = "Bit 11 - DMA mode switch"]
    #[inline(always)]
    #[must_use]
    pub fn dmaon(&mut self) -> DMAON_W<11> {
        DMAON_W::new(self)
    }
    #[doc = "Bit 12 - Flag indicating DMA last transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dmalst(&mut self) -> DMALST_W<12> {
        DMALST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
