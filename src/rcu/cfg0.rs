#[doc = "Register `CFG0` reader"]
pub struct R(crate::R<CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG0` writer"]
pub struct W(crate::W<CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG0_SPEC>;
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
impl From<crate::W<CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCS` reader - System clock switch"]
pub type SCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCS` writer - System clock switch"]
pub type SCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SCSS` reader - System clock switch status"]
pub type SCSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHBPSC` reader - AHB prescaler selection"]
pub type AHBPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHBPSC` writer - AHB prescaler selection"]
pub type AHBPSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `APB1PSC` reader - APB1 prescaler selection"]
pub type APB1PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APB1PSC` writer - APB1 prescaler selection"]
pub type APB1PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `APB2PSC` reader - APB2 prescaler selection"]
pub type APB2PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APB2PSC` writer - APB2 prescaler selection"]
pub type APB2PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `ADCPSC_1_0` reader - ADC clock prescaler selection"]
pub type ADCPSC_1_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCPSC_1_0` writer - ADC clock prescaler selection"]
pub type ADCPSC_1_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLLSEL` reader - PLL Clock Source Selection"]
pub type PLLSEL_R = crate::BitReader<bool>;
#[doc = "Field `PLLSEL` writer - PLL Clock Source Selection"]
pub type PLLSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, O>;
#[doc = "Field `PREDV0_LSB` reader - The LSB of PREDV0 division factor"]
pub type PREDV0_LSB_R = crate::BitReader<bool>;
#[doc = "Field `PREDV0_LSB` writer - The LSB of PREDV0 division factor"]
pub type PREDV0_LSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, O>;
#[doc = "Field `PLLMF_3_0` reader - The PLL clock multiplication factor"]
pub type PLLMF_3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLMF_3_0` writer - The PLL clock multiplication factor"]
pub type PLLMF_3_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `USBFSPSC` reader - USBFS clock prescaler selection"]
pub type USBFSPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBFSPSC` writer - USBFS clock prescaler selection"]
pub type USBFSPSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `CKOUT0SEL` reader - CKOUT0 Clock Source Selection"]
pub type CKOUT0SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKOUT0SEL` writer - CKOUT0 Clock Source Selection"]
pub type CKOUT0SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADCPSC_2` reader - Bit 2 of ADCPSC"]
pub type ADCPSC_2_R = crate::BitReader<bool>;
#[doc = "Field `ADCPSC_2` writer - Bit 2 of ADCPSC"]
pub type ADCPSC_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, O>;
#[doc = "Field `PLLMF_4` reader - Bit 4 of PLLMF"]
pub type PLLMF_4_R = crate::BitReader<bool>;
#[doc = "Field `PLLMF_4` writer - Bit 4 of PLLMF"]
pub type PLLMF_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn scss(&self) -> SCSS_R {
        SCSS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    pub fn ahbpsc(&self) -> AHBPSC_R {
        AHBPSC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    pub fn apb1psc(&self) -> APB1PSC_R {
        APB1PSC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    pub fn apb2psc(&self) -> APB2PSC_R {
        APB2PSC_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline(always)]
    pub fn adcpsc_1_0(&self) -> ADCPSC_1_0_R {
        ADCPSC_1_0_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    pub fn pllsel(&self) -> PLLSEL_R {
        PLLSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The LSB of PREDV0 division factor"]
    #[inline(always)]
    pub fn predv0_lsb(&self) -> PREDV0_LSB_R {
        PREDV0_LSB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - The PLL clock multiplication factor"]
    #[inline(always)]
    pub fn pllmf_3_0(&self) -> PLLMF_3_0_R {
        PLLMF_3_0_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - USBFS clock prescaler selection"]
    #[inline(always)]
    pub fn usbfspsc(&self) -> USBFSPSC_R {
        USBFSPSC_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - CKOUT0 Clock Source Selection"]
    #[inline(always)]
    pub fn ckout0sel(&self) -> CKOUT0SEL_R {
        CKOUT0SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Bit 2 of ADCPSC"]
    #[inline(always)]
    pub fn adcpsc_2(&self) -> ADCPSC_2_R {
        ADCPSC_2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Bit 4 of PLLMF"]
    #[inline(always)]
    pub fn pllmf_4(&self) -> PLLMF_4_R {
        PLLMF_4_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn scs(&mut self) -> SCS_W<0> {
        SCS_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn ahbpsc(&mut self) -> AHBPSC_W<4> {
        AHBPSC_W::new(self)
    }
    #[doc = "Bits 8:10 - APB1 prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn apb1psc(&mut self) -> APB1PSC_W<8> {
        APB1PSC_W::new(self)
    }
    #[doc = "Bits 11:13 - APB2 prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn apb2psc(&mut self) -> APB2PSC_W<11> {
        APB2PSC_W::new(self)
    }
    #[doc = "Bits 14:15 - ADC clock prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcpsc_1_0(&mut self) -> ADCPSC_1_0_W<14> {
        ADCPSC_1_0_W::new(self)
    }
    #[doc = "Bit 16 - PLL Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pllsel(&mut self) -> PLLSEL_W<16> {
        PLLSEL_W::new(self)
    }
    #[doc = "Bit 17 - The LSB of PREDV0 division factor"]
    #[inline(always)]
    #[must_use]
    pub fn predv0_lsb(&mut self) -> PREDV0_LSB_W<17> {
        PREDV0_LSB_W::new(self)
    }
    #[doc = "Bits 18:21 - The PLL clock multiplication factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllmf_3_0(&mut self) -> PLLMF_3_0_W<18> {
        PLLMF_3_0_W::new(self)
    }
    #[doc = "Bits 22:23 - USBFS clock prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbfspsc(&mut self) -> USBFSPSC_W<22> {
        USBFSPSC_W::new(self)
    }
    #[doc = "Bits 24:27 - CKOUT0 Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckout0sel(&mut self) -> CKOUT0SEL_W<24> {
        CKOUT0SEL_W::new(self)
    }
    #[doc = "Bit 28 - Bit 2 of ADCPSC"]
    #[inline(always)]
    #[must_use]
    pub fn adcpsc_2(&mut self) -> ADCPSC_2_W<28> {
        ADCPSC_2_W::new(self)
    }
    #[doc = "Bit 29 - Bit 4 of PLLMF"]
    #[inline(always)]
    #[must_use]
    pub fn pllmf_4(&mut self) -> PLLMF_4_W<29> {
        PLLMF_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register 0 (RCU_CFG0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](index.html) module"]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg0::R](R) reader structure"]
impl crate::Readable for CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg0::W](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
