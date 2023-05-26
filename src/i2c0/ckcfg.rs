#[doc = "Register `CKCFG` reader"]
pub struct R(crate::R<CKCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCFG` writer"]
pub struct W(crate::W<CKCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCFG_SPEC>;
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
impl From<crate::W<CKCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKC` reader - I2C Clock control in master mode"]
pub type CLKC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLKC` writer - I2C Clock control in master mode"]
pub type CLKC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CKCFG_SPEC, u16, u16, 12, O>;
#[doc = "Field `DTCY` reader - Duty cycle in fast mode"]
pub type DTCY_R = crate::BitReader<bool>;
#[doc = "Field `DTCY` writer - Duty cycle in fast mode"]
pub type DTCY_W<'a, const O: u8> = crate::BitWriter<'a, u16, CKCFG_SPEC, bool, O>;
#[doc = "Field `FAST` reader - I2C speed selection in master mode"]
pub type FAST_R = crate::BitReader<bool>;
#[doc = "Field `FAST` writer - I2C speed selection in master mode"]
pub type FAST_W<'a, const O: u8> = crate::BitWriter<'a, u16, CKCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - I2C Clock control in master mode"]
    #[inline(always)]
    pub fn clkc(&self) -> CLKC_R {
        CLKC_R::new(self.bits & 0x0fff)
    }
    #[doc = "Bit 14 - Duty cycle in fast mode"]
    #[inline(always)]
    pub fn dtcy(&self) -> DTCY_R {
        DTCY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C speed selection in master mode"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - I2C Clock control in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn clkc(&mut self) -> CLKC_W<0> {
        CLKC_W::new(self)
    }
    #[doc = "Bit 14 - Duty cycle in fast mode"]
    #[inline(always)]
    #[must_use]
    pub fn dtcy(&mut self) -> DTCY_W<14> {
        DTCY_W::new(self)
    }
    #[doc = "Bit 15 - I2C speed selection in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<15> {
        FAST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcfg](index.html) module"]
pub struct CKCFG_SPEC;
impl crate::RegisterSpec for CKCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ckcfg::R](R) reader structure"]
impl crate::Readable for CKCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcfg::W](W) writer structure"]
impl crate::Writable for CKCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCFG to value 0"]
impl crate::Resettable for CKCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
