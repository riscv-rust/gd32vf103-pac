#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `I2CCLK` reader - I2C Peripheral clock frequency"]
pub type I2CCLK_R = crate::FieldReader;
#[doc = "Field `I2CCLK` writer - I2C Peripheral clock frequency"]
pub type I2CCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVIE` reader - Event interrupt enable"]
pub type EVIE_R = crate::BitReader;
#[doc = "Field `EVIE` writer - Event interrupt enable"]
pub type EVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFIE` reader - Buffer interrupt enable"]
pub type BUFIE_R = crate::BitReader;
#[doc = "Field `BUFIE` writer - Buffer interrupt enable"]
pub type BUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAON` reader - DMA mode switch"]
pub type DMAON_R = crate::BitReader;
#[doc = "Field `DMAON` writer - DMA mode switch"]
pub type DMAON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMALST` reader - Flag indicating DMA last transfer"]
pub type DMALST_R = crate::BitReader;
#[doc = "Field `DMALST` writer - Flag indicating DMA last transfer"]
pub type DMALST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn i2cclk(&mut self) -> I2CCLK_W<CTL1_SPEC> {
        I2CCLK_W::new(self, 0)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTL1_SPEC> {
        ERRIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn evie(&mut self) -> EVIE_W<CTL1_SPEC> {
        EVIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufie(&mut self) -> BUFIE_W<CTL1_SPEC> {
        BUFIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA mode switch"]
    #[inline(always)]
    #[must_use]
    pub fn dmaon(&mut self) -> DMAON_W<CTL1_SPEC> {
        DMAON_W::new(self, 11)
    }
    #[doc = "Bit 12 - Flag indicating DMA last transfer"]
    #[inline(always)]
    #[must_use]
    pub fn dmalst(&mut self) -> DMALST_W<CTL1_SPEC> {
        DMALST_W::new(self, 12)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: u16 = 0;
}
