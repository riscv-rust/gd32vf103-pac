#[doc = "Register `APB2EN` reader"]
pub type R = crate::R<APB2EN_SPEC>;
#[doc = "Register `APB2EN` writer"]
pub type W = crate::W<APB2EN_SPEC>;
#[doc = "Field `AFEN` reader - Alternate function IO clock enable"]
pub type AFEN_R = crate::BitReader;
#[doc = "Field `AFEN` writer - Alternate function IO clock enable"]
pub type AFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAEN` reader - GPIO port A clock enable"]
pub type PAEN_R = crate::BitReader;
#[doc = "Field `PAEN` writer - GPIO port A clock enable"]
pub type PAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBEN` reader - GPIO port B clock enable"]
pub type PBEN_R = crate::BitReader;
#[doc = "Field `PBEN` writer - GPIO port B clock enable"]
pub type PBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCEN` reader - GPIO port C clock enable"]
pub type PCEN_R = crate::BitReader;
#[doc = "Field `PCEN` writer - GPIO port C clock enable"]
pub type PCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEN` reader - GPIO port D clock enable"]
pub type PDEN_R = crate::BitReader;
#[doc = "Field `PDEN` writer - GPIO port D clock enable"]
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEEN` reader - GPIO port E clock enable"]
pub type PEEN_R = crate::BitReader;
#[doc = "Field `PEEN` writer - GPIO port E clock enable"]
pub type PEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0EN` reader - ADC0 clock enable"]
pub type ADC0EN_R = crate::BitReader;
#[doc = "Field `ADC0EN` writer - ADC0 clock enable"]
pub type ADC0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1EN` reader - ADC1 clock enable"]
pub type ADC1EN_R = crate::BitReader;
#[doc = "Field `ADC1EN` writer - ADC1 clock enable"]
pub type ADC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0EN` reader - TIMER0 clock enable"]
pub type TIMER0EN_R = crate::BitReader;
#[doc = "Field `TIMER0EN` writer - TIMER0 clock enable"]
pub type TIMER0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0EN` reader - SPI0 clock enable"]
pub type SPI0EN_R = crate::BitReader;
#[doc = "Field `SPI0EN` writer - SPI0 clock enable"]
pub type SPI0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0EN` reader - USART0 clock enable"]
pub type USART0EN_R = crate::BitReader;
#[doc = "Field `USART0EN` writer - USART0 clock enable"]
pub type USART0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alternate function IO clock enable"]
    #[inline(always)]
    pub fn afen(&self) -> AFEN_R {
        AFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO port A clock enable"]
    #[inline(always)]
    pub fn paen(&self) -> PAEN_R {
        PAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO port B clock enable"]
    #[inline(always)]
    pub fn pben(&self) -> PBEN_R {
        PBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO port C clock enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO port D clock enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO port E clock enable"]
    #[inline(always)]
    pub fn peen(&self) -> PEEN_R {
        PEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC0 clock enable"]
    #[inline(always)]
    pub fn adc0en(&self) -> ADC0EN_R {
        ADC0EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER0 clock enable"]
    #[inline(always)]
    pub fn timer0en(&self) -> TIMER0EN_R {
        TIMER0EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    pub fn spi0en(&self) -> SPI0EN_R {
        SPI0EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    pub fn usart0en(&self) -> USART0EN_R {
        USART0EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function IO clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn afen(&mut self) -> AFEN_W<APB2EN_SPEC> {
        AFEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - GPIO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn paen(&mut self) -> PAEN_W<APB2EN_SPEC> {
        PAEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pben(&mut self) -> PBEN_W<APB2EN_SPEC> {
        PBEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<APB2EN_SPEC> {
        PCEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<APB2EN_SPEC> {
        PDEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn peen(&mut self) -> PEEN_W<APB2EN_SPEC> {
        PEEN_W::new(self, 6)
    }
    #[doc = "Bit 9 - ADC0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0en(&mut self) -> ADC0EN_W<APB2EN_SPEC> {
        ADC0EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> ADC1EN_W<APB2EN_SPEC> {
        ADC1EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - TIMER0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer0en(&mut self) -> TIMER0EN_W<APB2EN_SPEC> {
        TIMER0EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi0en(&mut self) -> SPI0EN_W<APB2EN_SPEC> {
        SPI0EN_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0en(&mut self) -> USART0EN_W<APB2EN_SPEC> {
        USART0EN_W::new(self, 14)
    }
}
#[doc = "APB2 clock enable register (RCU_APB2EN)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2EN_SPEC;
impl crate::RegisterSpec for APB2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2en::R`](R) reader structure"]
impl crate::Readable for APB2EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2en::W`](W) writer structure"]
impl crate::Writable for APB2EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB2EN to value 0"]
impl crate::Resettable for APB2EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
