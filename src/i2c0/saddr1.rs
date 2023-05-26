#[doc = "Register `SADDR1` reader"]
pub struct R(crate::R<SADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADDR1` writer"]
pub struct W(crate::W<SADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SADDR1_SPEC>;
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
impl From<crate::W<SADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUADEN` reader - Dual-Address mode switch"]
pub type DUADEN_R = crate::BitReader<bool>;
#[doc = "Field `DUADEN` writer - Dual-Address mode switch"]
pub type DUADEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, SADDR1_SPEC, bool, O>;
#[doc = "Field `ADDRESS2` reader - Second I2C address for the slave in Dual-Address mode"]
pub type ADDRESS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRESS2` writer - Second I2C address for the slave in Dual-Address mode"]
pub type ADDRESS2_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SADDR1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Dual-Address mode switch"]
    #[inline(always)]
    pub fn duaden(&self) -> DUADEN_R {
        DUADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Second I2C address for the slave in Dual-Address mode"]
    #[inline(always)]
    pub fn address2(&self) -> ADDRESS2_R {
        ADDRESS2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dual-Address mode switch"]
    #[inline(always)]
    #[must_use]
    pub fn duaden(&mut self) -> DUADEN_W<0> {
        DUADEN_W::new(self)
    }
    #[doc = "Bits 1:7 - Second I2C address for the slave in Dual-Address mode"]
    #[inline(always)]
    #[must_use]
    pub fn address2(&mut self) -> ADDRESS2_W<1> {
        ADDRESS2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave address register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddr1](index.html) module"]
pub struct SADDR1_SPEC;
impl crate::RegisterSpec for SADDR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [saddr1::R](R) reader structure"]
impl crate::Readable for SADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saddr1::W](W) writer structure"]
impl crate::Writable for SADDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SADDR1 to value 0"]
impl crate::Resettable for SADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
