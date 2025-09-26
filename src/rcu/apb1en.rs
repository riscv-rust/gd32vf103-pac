#[doc = "Register `APB1EN` reader"]
pub type R = crate::R<APB1EN_SPEC>;
#[doc = "Register `APB1EN` writer"]
pub type W = crate::W<APB1EN_SPEC>;
#[doc = "Field `TIMER1EN` reader - TIMER1 timer clock enable"]
pub type TIMER1EN_R = crate::BitReader;
#[doc = "Field `TIMER1EN` writer - TIMER1 timer clock enable"]
pub type TIMER1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2EN` reader - TIMER2 timer clock enable"]
pub type TIMER2EN_R = crate::BitReader;
#[doc = "Field `TIMER2EN` writer - TIMER2 timer clock enable"]
pub type TIMER2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3EN` reader - TIMER3 timer clock enable"]
pub type TIMER3EN_R = crate::BitReader;
#[doc = "Field `TIMER3EN` writer - TIMER3 timer clock enable"]
pub type TIMER3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4EN` reader - TIMER4 timer clock enable"]
pub type TIMER4EN_R = crate::BitReader;
#[doc = "Field `TIMER4EN` writer - TIMER4 timer clock enable"]
pub type TIMER4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER5EN` reader - TIMER5 timer clock enable"]
pub type TIMER5EN_R = crate::BitReader;
#[doc = "Field `TIMER5EN` writer - TIMER5 timer clock enable"]
pub type TIMER5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER6EN` reader - TIMER6 timer clock enable"]
pub type TIMER6EN_R = crate::BitReader;
#[doc = "Field `TIMER6EN` writer - TIMER6 timer clock enable"]
pub type TIMER6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGTEN` reader - Window watchdog timer clock enable"]
pub type WWDGTEN_R = crate::BitReader;
#[doc = "Field `WWDGTEN` writer - Window watchdog timer clock enable"]
pub type WWDGTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub type SPI2EN_R = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type USART1EN_R = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub type USART2EN_R = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3EN` reader - UART3 clock enable"]
pub type UART3EN_R = crate::BitReader;
#[doc = "Field `UART3EN` writer - UART3 clock enable"]
pub type UART3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4EN` reader - UART4 clock enable"]
pub type UART4EN_R = crate::BitReader;
#[doc = "Field `UART4EN` writer - UART4 clock enable"]
pub type UART4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0EN` reader - I2C0 clock enable"]
pub type I2C0EN_R = crate::BitReader;
#[doc = "Field `I2C0EN` writer - I2C0 clock enable"]
pub type I2C0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub type I2C1EN_R = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN0EN` reader - CAN0 clock enable"]
pub type CAN0EN_R = crate::BitReader;
#[doc = "Field `CAN0EN` writer - CAN0 clock enable"]
pub type CAN0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1EN` reader - CAN1 clock enable"]
pub type CAN1EN_R = crate::BitReader;
#[doc = "Field `CAN1EN` writer - CAN1 clock enable"]
pub type CAN1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPIEN` reader - Backup interface clock enable"]
pub type BKPIEN_R = crate::BitReader;
#[doc = "Field `BKPIEN` writer - Backup interface clock enable"]
pub type BKPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMUEN` reader - Power control clock enable"]
pub type PMUEN_R = crate::BitReader;
#[doc = "Field `PMUEN` writer - Power control clock enable"]
pub type PMUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACEN` reader - DAC clock enable"]
pub type DACEN_R = crate::BitReader;
#[doc = "Field `DACEN` writer - DAC clock enable"]
pub type DACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIMER1 timer clock enable"]
    #[inline(always)]
    pub fn timer1en(&self) -> TIMER1EN_R {
        TIMER1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    pub fn timer2en(&self) -> TIMER2EN_R {
        TIMER2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIMER3 timer clock enable"]
    #[inline(always)]
    pub fn timer3en(&self) -> TIMER3EN_R {
        TIMER3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIMER4 timer clock enable"]
    #[inline(always)]
    pub fn timer4en(&self) -> TIMER4EN_R {
        TIMER4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    pub fn timer5en(&self) -> TIMER5EN_R {
        TIMER5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIMER6 timer clock enable"]
    #[inline(always)]
    pub fn timer6en(&self) -> TIMER6EN_R {
        TIMER6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdgten(&self) -> WWDGTEN_R {
        WWDGTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART3 clock enable"]
    #[inline(always)]
    pub fn uart3en(&self) -> UART3EN_R {
        UART3EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    pub fn i2c0en(&self) -> I2C0EN_R {
        I2C0EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN0 clock enable"]
    #[inline(always)]
    pub fn can0en(&self) -> CAN0EN_R {
        CAN0EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1en(&self) -> CAN1EN_R {
        CAN1EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Backup interface clock enable"]
    #[inline(always)]
    pub fn bkpien(&self) -> BKPIEN_R {
        BKPIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power control clock enable"]
    #[inline(always)]
    pub fn pmuen(&self) -> PMUEN_R {
        PMUEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMER1 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer1en(&mut self) -> TIMER1EN_W<APB1EN_SPEC> {
        TIMER1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIMER2 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer2en(&mut self) -> TIMER2EN_W<APB1EN_SPEC> {
        TIMER2EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIMER3 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer3en(&mut self) -> TIMER3EN_W<APB1EN_SPEC> {
        TIMER3EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIMER4 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer4en(&mut self) -> TIMER4EN_W<APB1EN_SPEC> {
        TIMER4EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIMER5 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer5en(&mut self) -> TIMER5EN_W<APB1EN_SPEC> {
        TIMER5EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIMER6 timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer6en(&mut self) -> TIMER6EN_W<APB1EN_SPEC> {
        TIMER6EN_W::new(self, 5)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgten(&mut self) -> WWDGTEN_W<APB1EN_SPEC> {
        WWDGTEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<APB1EN_SPEC> {
        SPI1EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APB1EN_SPEC> {
        SPI2EN_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<APB1EN_SPEC> {
        USART1EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART2 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APB1EN_SPEC> {
        USART2EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - UART3 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart3en(&mut self) -> UART3EN_W<APB1EN_SPEC> {
        UART3EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - UART4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<APB1EN_SPEC> {
        UART4EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0en(&mut self) -> I2C0EN_W<APB1EN_SPEC> {
        I2C0EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1EN_SPEC> {
        I2C1EN_W::new(self, 22)
    }
    #[doc = "Bit 25 - CAN0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can0en(&mut self) -> CAN0EN_W<APB1EN_SPEC> {
        CAN0EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - CAN1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn can1en(&mut self) -> CAN1EN_W<APB1EN_SPEC> {
        CAN1EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Backup interface clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpien(&mut self) -> BKPIEN_W<APB1EN_SPEC> {
        BKPIEN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Power control clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmuen(&mut self) -> PMUEN_W<APB1EN_SPEC> {
        PMUEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<APB1EN_SPEC> {
        DACEN_W::new(self, 29)
    }
}
#[doc = "APB1 clock enable register (RCU_APB1EN)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1EN_SPEC;
impl crate::RegisterSpec for APB1EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1en::R`](R) reader structure"]
impl crate::Readable for APB1EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1en::W`](W) writer structure"]
impl crate::Writable for APB1EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1EN to value 0"]
impl crate::Resettable for APB1EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
