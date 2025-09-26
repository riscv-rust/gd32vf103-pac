#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<APB2RST_SPEC>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<APB2RST_SPEC>;
#[doc = "Field `AFRST` reader - Alternate function I/O reset"]
pub type AFRST_R = crate::BitReader;
#[doc = "Field `AFRST` writer - Alternate function I/O reset"]
pub type AFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub type PARST_R = crate::BitReader;
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub type PARST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub type PBRST_R = crate::BitReader;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub type PBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub type PCRST_R = crate::BitReader;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub type PCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDRST` reader - GPIO port D reset"]
pub type PDRST_R = crate::BitReader;
#[doc = "Field `PDRST` writer - GPIO port D reset"]
pub type PDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERST` reader - GPIO port E reset"]
pub type PERST_R = crate::BitReader;
#[doc = "Field `PERST` writer - GPIO port E reset"]
pub type PERST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0RST` reader - ADC0 reset"]
pub type ADC0RST_R = crate::BitReader;
#[doc = "Field `ADC0RST` writer - ADC0 reset"]
pub type ADC0RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1RST` reader - ADC1 reset"]
pub type ADC1RST_R = crate::BitReader;
#[doc = "Field `ADC1RST` writer - ADC1 reset"]
pub type ADC1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0RST` reader - Timer 0 reset"]
pub type TIMER0RST_R = crate::BitReader;
#[doc = "Field `TIMER0RST` writer - Timer 0 reset"]
pub type TIMER0RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0RST` reader - SPI0 reset"]
pub type SPI0RST_R = crate::BitReader;
#[doc = "Field `SPI0RST` writer - SPI0 reset"]
pub type SPI0RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0RST` reader - USART0 Reset"]
pub type USART0RST_R = crate::BitReader;
#[doc = "Field `USART0RST` writer - USART0 Reset"]
pub type USART0RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    pub fn afrst(&self) -> AFRST_R {
        AFRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO port A reset"]
    #[inline(always)]
    pub fn parst(&self) -> PARST_R {
        PARST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO port B reset"]
    #[inline(always)]
    pub fn pbrst(&self) -> PBRST_R {
        PBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO port C reset"]
    #[inline(always)]
    pub fn pcrst(&self) -> PCRST_R {
        PCRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO port D reset"]
    #[inline(always)]
    pub fn pdrst(&self) -> PDRST_R {
        PDRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO port E reset"]
    #[inline(always)]
    pub fn perst(&self) -> PERST_R {
        PERST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC0 reset"]
    #[inline(always)]
    pub fn adc0rst(&self) -> ADC0RST_R {
        ADC0RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer 0 reset"]
    #[inline(always)]
    pub fn timer0rst(&self) -> TIMER0RST_R {
        TIMER0RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 reset"]
    #[inline(always)]
    pub fn spi0rst(&self) -> SPI0RST_R {
        SPI0RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    pub fn usart0rst(&self) -> USART0RST_R {
        USART0RST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    #[must_use]
    pub fn afrst(&mut self) -> AFRST_W<APB2RST_SPEC> {
        AFRST_W::new(self, 0)
    }
    #[doc = "Bit 2 - GPIO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> PARST_W<APB2RST_SPEC> {
        PARST_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PBRST_W<APB2RST_SPEC> {
        PBRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PCRST_W<APB2RST_SPEC> {
        PCRST_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst(&mut self) -> PDRST_W<APB2RST_SPEC> {
        PDRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn perst(&mut self) -> PERST_W<APB2RST_SPEC> {
        PERST_W::new(self, 6)
    }
    #[doc = "Bit 9 - ADC0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc0rst(&mut self) -> ADC0RST_W<APB2RST_SPEC> {
        ADC0RST_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<APB2RST_SPEC> {
        ADC1RST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Timer 0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer0rst(&mut self) -> TIMER0RST_W<APB2RST_SPEC> {
        TIMER0RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi0rst(&mut self) -> SPI0RST_W<APB2RST_SPEC> {
        SPI0RST_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart0rst(&mut self) -> USART0RST_W<APB2RST_SPEC> {
        USART0RST_W::new(self, 14)
    }
}
#[doc = "APB2 reset register (RCU_APB2RST)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RST_SPEC;
impl crate::RegisterSpec for APB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for APB2RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for APB2RST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for APB2RST_SPEC {
    const RESET_VALUE: u32 = 0;
}
