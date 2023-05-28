#[doc = "Register `APB2EN` reader"]
pub struct R(crate::R<APB2EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2EN` writer"]
pub struct W(crate::W<APB2EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2EN_SPEC>;
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
impl From<crate::W<APB2EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFEN` reader - Alternate function IO clock enable"]
pub type AFEN_R = crate::BitReader<bool>;
#[doc = "Field `AFEN` writer - Alternate function IO clock enable"]
pub type AFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2EN_SPEC, bool, O>;
#[doc = "Field `PAEN` reader - GPIO port A clock enable"]
pub type PAEN_R = crate::BitReader<bool>;
#[doc = "Field `PAEN` writer - GPIO port A clock enable"]
pub type PAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2EN_SPEC, bool, O>;
#[doc = "Field `PBEN` reader - GPIO port B clock enable"]
pub type PBEN_R = crate::BitReader<bool>;
#[doc = "Field `PBEN` writer - GPIO port B clock enable"]
pub type PBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2EN_SPEC, bool, O>;
#[doc = "Field `PCEN` reader - GPIO port C clock enable"]
pub type PCEN_R = crate::BitReader<bool>;
#[doc = "Field `PCEN` writer - GPIO port C clock enable"]
pub type PCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2EN_SPEC, bool, O>;
#[doc = "Field `PDEN` reader - GPIO port D clock enable"]
pub type PDEN_R = crate::BitReader<bool>;
#[doc = "Field `PDEN` writer - GPIO port D clock enable"]
pub type PDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2EN_SPEC, bool, O>;
#[doc = "Field `PEEN` reader - GPIO port E clock enable"]
pub type PEEN_R = crate::BitReader<bool>;
#[doc = "Field `PEEN` writer - GPIO port E clock enable"]
pub type PEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2EN_SPEC, bool, O>;
#[doc = "Field `ADC0EN` reader - ADC0 clock enable"]
pub type ADC0EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC0EN` writer - ADC0 clock enable"]
pub type ADC0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2EN_SPEC, bool, O>;
#[doc = "Field `ADC1EN` reader - ADC1 clock enable"]
pub type ADC1EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC1EN` writer - ADC1 clock enable"]
pub type ADC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2EN_SPEC, bool, O>;
#[doc = "Field `TIMER0EN` reader - TIMER0 clock enable"]
pub type TIMER0EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0EN` writer - TIMER0 clock enable"]
pub type TIMER0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2EN_SPEC, bool, O>;
#[doc = "Field `SPI0EN` reader - SPI0 clock enable"]
pub type SPI0EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI0EN` writer - SPI0 clock enable"]
pub type SPI0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2EN_SPEC, bool, O>;
#[doc = "Field `USART0EN` reader - USART0 clock enable"]
pub type USART0EN_R = crate::BitReader<bool>;
#[doc = "Field `USART0EN` writer - USART0 clock enable"]
pub type USART0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2EN_SPEC, bool, O>;
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
    pub fn afen(&mut self) -> AFEN_W<0> {
        AFEN_W::new(self)
    }
    #[doc = "Bit 2 - GPIO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn paen(&mut self) -> PAEN_W<2> {
        PAEN_W::new(self)
    }
    #[doc = "Bit 3 - GPIO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pben(&mut self) -> PBEN_W<3> {
        PBEN_W::new(self)
    }
    #[doc = "Bit 4 - GPIO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<4> {
        PCEN_W::new(self)
    }
    #[doc = "Bit 5 - GPIO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<5> {
        PDEN_W::new(self)
    }
    #[doc = "Bit 6 - GPIO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn peen(&mut self) -> PEEN_W<6> {
        PEEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0en(&mut self) -> ADC0EN_W<9> {
        ADC0EN_W::new(self)
    }
    #[doc = "Bit 10 - ADC1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1en(&mut self) -> ADC1EN_W<10> {
        ADC1EN_W::new(self)
    }
    #[doc = "Bit 11 - TIMER0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer0en(&mut self) -> TIMER0EN_W<11> {
        TIMER0EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi0en(&mut self) -> SPI0EN_W<12> {
        SPI0EN_W::new(self)
    }
    #[doc = "Bit 14 - USART0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0en(&mut self) -> USART0EN_W<14> {
        USART0EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 clock enable register (RCU_APB2EN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2en](index.html) module"]
pub struct APB2EN_SPEC;
impl crate::RegisterSpec for APB2EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2en::R](R) reader structure"]
impl crate::Readable for APB2EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2en::W](W) writer structure"]
impl crate::Writable for APB2EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2EN to value 0"]
impl crate::Resettable for APB2EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
