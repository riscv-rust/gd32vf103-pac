#[doc = "Register `SADDR0` reader"]
pub struct R(crate::R<SADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADDR0` writer"]
pub struct W(crate::W<SADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SADDR0_SPEC>;
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
impl From<crate::W<SADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS0` reader - Bit 0 of a 10-bit address"]
pub type ADDRESS0_R = crate::BitReader<bool>;
#[doc = "Field `ADDRESS0` writer - Bit 0 of a 10-bit address"]
pub type ADDRESS0_W<'a, const O: u8> = crate::BitWriter<'a, u16, SADDR0_SPEC, bool, O>;
#[doc = "Field `ADDRESS7_1` reader - 7-bit address or bits 7:1 of a 10-bit address"]
pub type ADDRESS7_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRESS7_1` writer - 7-bit address or bits 7:1 of a 10-bit address"]
pub type ADDRESS7_1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SADDR0_SPEC, u8, u8, 7, O>;
#[doc = "Field `ADDRESS9_8` reader - Highest two bits of a 10-bit address"]
pub type ADDRESS9_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRESS9_8` writer - Highest two bits of a 10-bit address"]
pub type ADDRESS9_8_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SADDR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADDFORMAT` reader - Address mode for the I2C slave"]
pub type ADDFORMAT_R = crate::BitReader<bool>;
#[doc = "Field `ADDFORMAT` writer - Address mode for the I2C slave"]
pub type ADDFORMAT_W<'a, const O: u8> = crate::BitWriter<'a, u16, SADDR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Bit 0 of a 10-bit address"]
    #[inline(always)]
    pub fn address0(&self) -> ADDRESS0_R {
        ADDRESS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7-bit address or bits 7:1 of a 10-bit address"]
    #[inline(always)]
    pub fn address7_1(&self) -> ADDRESS7_1_R {
        ADDRESS7_1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Highest two bits of a 10-bit address"]
    #[inline(always)]
    pub fn address9_8(&self) -> ADDRESS9_8_R {
        ADDRESS9_8_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Address mode for the I2C slave"]
    #[inline(always)]
    pub fn addformat(&self) -> ADDFORMAT_R {
        ADDFORMAT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit 0 of a 10-bit address"]
    #[inline(always)]
    #[must_use]
    pub fn address0(&mut self) -> ADDRESS0_W<0> {
        ADDRESS0_W::new(self)
    }
    #[doc = "Bits 1:7 - 7-bit address or bits 7:1 of a 10-bit address"]
    #[inline(always)]
    #[must_use]
    pub fn address7_1(&mut self) -> ADDRESS7_1_W<1> {
        ADDRESS7_1_W::new(self)
    }
    #[doc = "Bits 8:9 - Highest two bits of a 10-bit address"]
    #[inline(always)]
    #[must_use]
    pub fn address9_8(&mut self) -> ADDRESS9_8_W<8> {
        ADDRESS9_8_W::new(self)
    }
    #[doc = "Bit 15 - Address mode for the I2C slave"]
    #[inline(always)]
    #[must_use]
    pub fn addformat(&mut self) -> ADDFORMAT_W<15> {
        ADDFORMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave address register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddr0](index.html) module"]
pub struct SADDR0_SPEC;
impl crate::RegisterSpec for SADDR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [saddr0::R](R) reader structure"]
impl crate::Readable for SADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saddr0::W](W) writer structure"]
impl crate::Writable for SADDR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SADDR0 to value 0"]
impl crate::Resettable for SADDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
