#[doc = "Register `APB2RST` reader"]
pub struct R(crate::R<APB2RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2RST` writer"]
pub struct W(crate::W<APB2RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2RST_SPEC>;
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
impl From<crate::W<APB2RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFRST` reader - Alternate function I/O reset"]
pub type AFRST_R = crate::BitReader<bool>;
#[doc = "Field `AFRST` writer - Alternate function I/O reset"]
pub type AFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, O>;
#[doc = "Field `PARST` reader - GPIO port A reset"]
pub type PARST_R = crate::BitReader<bool>;
#[doc = "Field `PARST` writer - GPIO port A reset"]
pub type PARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, O>;
#[doc = "Field `PBRST` reader - GPIO port B reset"]
pub type PBRST_R = crate::BitReader<bool>;
#[doc = "Field `PBRST` writer - GPIO port B reset"]
pub type PBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, O>;
#[doc = "Field `PCRST` reader - GPIO port C reset"]
pub type PCRST_R = crate::BitReader<bool>;
#[doc = "Field `PCRST` writer - GPIO port C reset"]
pub type PCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, O>;
#[doc = "Field `PDRST` reader - GPIO port D reset"]
pub type PDRST_R = crate::BitReader<bool>;
#[doc = "Field `PDRST` writer - GPIO port D reset"]
pub type PDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, O>;
#[doc = "Field `PERST` reader - GPIO port E reset"]
pub type PERST_R = crate::BitReader<bool>;
#[doc = "Field `PERST` writer - GPIO port E reset"]
pub type PERST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, O>;
#[doc = "Field `ADC0RST` reader - ADC0 reset"]
pub type ADC0RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC0RST` writer - ADC0 reset"]
pub type ADC0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, O>;
#[doc = "Field `ADC1RST` reader - ADC1 reset"]
pub type ADC1RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC1RST` writer - ADC1 reset"]
pub type ADC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, O>;
#[doc = "Field `TIMER0RST` reader - Timer 0 reset"]
pub type TIMER0RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0RST` writer - Timer 0 reset"]
pub type TIMER0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, O>;
#[doc = "Field `SPI0RST` reader - SPI0 reset"]
pub type SPI0RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI0RST` writer - SPI0 reset"]
pub type SPI0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, O>;
#[doc = "Field `USART0RST` reader - USART0 Reset"]
pub type USART0RST_R = crate::BitReader<bool>;
#[doc = "Field `USART0RST` writer - USART0 Reset"]
pub type USART0RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RST_SPEC, bool, O>;
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
    pub fn afrst(&mut self) -> AFRST_W<0> {
        AFRST_W::new(self)
    }
    #[doc = "Bit 2 - GPIO port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> PARST_W<2> {
        PARST_W::new(self)
    }
    #[doc = "Bit 3 - GPIO port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PBRST_W<3> {
        PBRST_W::new(self)
    }
    #[doc = "Bit 4 - GPIO port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PCRST_W<4> {
        PCRST_W::new(self)
    }
    #[doc = "Bit 5 - GPIO port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst(&mut self) -> PDRST_W<5> {
        PDRST_W::new(self)
    }
    #[doc = "Bit 6 - GPIO port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn perst(&mut self) -> PERST_W<6> {
        PERST_W::new(self)
    }
    #[doc = "Bit 9 - ADC0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc0rst(&mut self) -> ADC0RST_W<9> {
        ADC0RST_W::new(self)
    }
    #[doc = "Bit 10 - ADC1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc1rst(&mut self) -> ADC1RST_W<10> {
        ADC1RST_W::new(self)
    }
    #[doc = "Bit 11 - Timer 0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn timer0rst(&mut self) -> TIMER0RST_W<11> {
        TIMER0RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI0 reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi0rst(&mut self) -> SPI0RST_W<12> {
        SPI0RST_W::new(self)
    }
    #[doc = "Bit 14 - USART0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart0rst(&mut self) -> USART0RST_W<14> {
        USART0RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 reset register (RCU_APB2RST)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rst](index.html) module"]
pub struct APB2RST_SPEC;
impl crate::RegisterSpec for APB2RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2rst::R](R) reader structure"]
impl crate::Readable for APB2RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2rst::W](W) writer structure"]
impl crate::Writable for APB2RST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for APB2RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
