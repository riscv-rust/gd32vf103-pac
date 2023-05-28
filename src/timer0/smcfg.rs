#[doc = "Register `SMCFG` reader"]
pub struct R(crate::R<SMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCFG` writer"]
pub struct W(crate::W<SMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCFG_SPEC>;
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
impl From<crate::W<SMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMC` reader - Slave mode selection"]
pub type SMC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMC` writer - Slave mode selection"]
pub type SMC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `TRGS` reader - Trigger selection"]
pub type TRGS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGS` writer - Trigger selection"]
pub type TRGS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader<bool>;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCFG_SPEC, bool, O>;
#[doc = "Field `ETFC` reader - External trigger filter control"]
pub type ETFC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETFC` writer - External trigger filter control"]
pub type ETFC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `ETPSC` reader - External trigger prescaler"]
pub type ETPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETPSC` writer - External trigger prescaler"]
pub type ETPSC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SMC1` reader - Part of SMC for enable External clock mode1"]
pub type SMC1_R = crate::BitReader<bool>;
#[doc = "Field `SMC1` writer - Part of SMC for enable External clock mode1"]
pub type SMC1_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCFG_SPEC, bool, O>;
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type ETP_R = crate::BitReader<bool>;
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type ETP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn trgs(&self) -> TRGS_R {
        TRGS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter control"]
    #[inline(always)]
    pub fn etfc(&self) -> ETFC_R {
        ETFC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etpsc(&self) -> ETPSC_R {
        ETPSC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Part of SMC for enable External clock mode1"]
    #[inline(always)]
    pub fn smc1(&self) -> SMC1_R {
        SMC1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn smc(&mut self) -> SMC_W<0> {
        SMC_W::new(self)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgs(&mut self) -> TRGS_W<4> {
        TRGS_W::new(self)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<7> {
        MSM_W::new(self)
    }
    #[doc = "Bits 8:11 - External trigger filter control"]
    #[inline(always)]
    #[must_use]
    pub fn etfc(&mut self) -> ETFC_W<8> {
        ETFC_W::new(self)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn etpsc(&mut self) -> ETPSC_W<12> {
        ETPSC_W::new(self)
    }
    #[doc = "Bit 14 - Part of SMC for enable External clock mode1"]
    #[inline(always)]
    #[must_use]
    pub fn smc1(&mut self) -> SMC1_W<14> {
        SMC1_W::new(self)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<15> {
        ETP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "slave mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcfg](index.html) module"]
pub struct SMCFG_SPEC;
impl crate::RegisterSpec for SMCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcfg::R](R) reader structure"]
impl crate::Readable for SMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcfg::W](W) writer structure"]
impl crate::Writable for SMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCFG to value 0"]
impl crate::Resettable for SMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
