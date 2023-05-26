#[doc = "Register `PCF1` reader"]
pub struct R(crate::R<PCF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCF1` writer"]
pub struct W(crate::W<PCF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCF1_SPEC>;
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
impl From<crate::W<PCF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXMC_NADV` reader - EXMC_NADV connect/disconnect"]
pub type EXMC_NADV_R = crate::BitReader<bool>;
#[doc = "Field `EXMC_NADV` writer - EXMC_NADV connect/disconnect"]
pub type EXMC_NADV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCF1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    pub fn exmc_nadv(&self) -> EXMC_NADV_R {
        EXMC_NADV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn exmc_nadv(&mut self) -> EXMC_NADV_W<10> {
        EXMC_NADV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO port configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcf1](index.html) module"]
pub struct PCF1_SPEC;
impl crate::RegisterSpec for PCF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcf1::R](R) reader structure"]
impl crate::Readable for PCF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcf1::W](W) writer structure"]
impl crate::Writable for PCF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCF1 to value 0"]
impl crate::Resettable for PCF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
