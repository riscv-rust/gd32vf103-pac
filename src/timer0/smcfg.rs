#[doc = "Register `SMCFG` reader"]
pub type R = crate::R<SMCFG_SPEC>;
#[doc = "Register `SMCFG` writer"]
pub type W = crate::W<SMCFG_SPEC>;
#[doc = "Field `SMC` reader - Slave mode selection"]
pub type SMC_R = crate::FieldReader;
#[doc = "Field `SMC` writer - Slave mode selection"]
pub type SMC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRGS` reader - Trigger selection"]
pub type TRGS_R = crate::FieldReader;
#[doc = "Field `TRGS` writer - Trigger selection"]
pub type TRGS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETFC` reader - External trigger filter control"]
pub type ETFC_R = crate::FieldReader;
#[doc = "Field `ETFC` writer - External trigger filter control"]
pub type ETFC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPSC` reader - External trigger prescaler"]
pub type ETPSC_R = crate::FieldReader;
#[doc = "Field `ETPSC` writer - External trigger prescaler"]
pub type ETPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMC1` reader - Part of SMC for enable External clock mode1"]
pub type SMC1_R = crate::BitReader;
#[doc = "Field `SMC1` writer - Part of SMC for enable External clock mode1"]
pub type SMC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type ETP_R = crate::BitReader;
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn smc(&mut self) -> SMC_W<SMCFG_SPEC> {
        SMC_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgs(&mut self) -> TRGS_W<SMCFG_SPEC> {
        TRGS_W::new(self, 4)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<SMCFG_SPEC> {
        MSM_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - External trigger filter control"]
    #[inline(always)]
    #[must_use]
    pub fn etfc(&mut self) -> ETFC_W<SMCFG_SPEC> {
        ETFC_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn etpsc(&mut self) -> ETPSC_W<SMCFG_SPEC> {
        ETPSC_W::new(self, 12)
    }
    #[doc = "Bit 14 - Part of SMC for enable External clock mode1"]
    #[inline(always)]
    #[must_use]
    pub fn smc1(&mut self) -> SMC1_W<SMCFG_SPEC> {
        SMC1_W::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<SMCFG_SPEC> {
        ETP_W::new(self, 15)
    }
}
#[doc = "slave mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMCFG_SPEC;
impl crate::RegisterSpec for SMCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`smcfg::R`](R) reader structure"]
impl crate::Readable for SMCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smcfg::W`](W) writer structure"]
impl crate::Writable for SMCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SMCFG to value 0"]
impl crate::Resettable for SMCFG_SPEC {
    const RESET_VALUE: u16 = 0;
}
