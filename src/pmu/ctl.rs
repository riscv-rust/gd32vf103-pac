#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `LDOLP` reader - LDO Low Power Mode"]
pub type LDOLP_R = crate::BitReader;
#[doc = "Field `LDOLP` writer - LDO Low Power Mode"]
pub type LDOLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBMOD` reader - Standby Mode"]
pub type STBMOD_R = crate::BitReader;
#[doc = "Field `STBMOD` writer - Standby Mode"]
pub type STBMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WURST` reader - Wakeup Flag Reset"]
pub type WURST_R = crate::BitReader;
#[doc = "Field `WURST` writer - Wakeup Flag Reset"]
pub type WURST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBRST` reader - Standby Flag Reset"]
pub type STBRST_R = crate::BitReader;
#[doc = "Field `STBRST` writer - Standby Flag Reset"]
pub type STBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVDEN` reader - Low Voltage Detector Enable"]
pub type LVDEN_R = crate::BitReader;
#[doc = "Field `LVDEN` writer - Low Voltage Detector Enable"]
pub type LVDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVDT` reader - Low Voltage Detector Threshold"]
pub type LVDT_R = crate::FieldReader;
#[doc = "Field `LVDT` writer - Low Voltage Detector Threshold"]
pub type LVDT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BKPWEN` reader - Backup Domain Write Enable"]
pub type BKPWEN_R = crate::BitReader;
#[doc = "Field `BKPWEN` writer - Backup Domain Write Enable"]
pub type BKPWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    pub fn ldolp(&self) -> LDOLP_R {
        LDOLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    pub fn stbmod(&self) -> STBMOD_R {
        STBMOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    pub fn wurst(&self) -> WURST_R {
        WURST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    pub fn stbrst(&self) -> STBRST_R {
        STBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    pub fn lvden(&self) -> LVDEN_R {
        LVDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    pub fn lvdt(&self) -> LVDT_R {
        LVDT_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    pub fn bkpwen(&self) -> BKPWEN_R {
        BKPWEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LDO Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ldolp(&mut self) -> LDOLP_W<CTL_SPEC> {
        LDOLP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Standby Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stbmod(&mut self) -> STBMOD_W<CTL_SPEC> {
        STBMOD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wurst(&mut self) -> WURST_W<CTL_SPEC> {
        WURST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    #[must_use]
    pub fn stbrst(&mut self) -> STBRST_W<CTL_SPEC> {
        STBRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Low Voltage Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvden(&mut self) -> LVDEN_W<CTL_SPEC> {
        LVDEN_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Low Voltage Detector Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lvdt(&mut self) -> LVDT_W<CTL_SPEC> {
        LVDT_W::new(self, 5)
    }
    #[doc = "Bit 8 - Backup Domain Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpwen(&mut self) -> BKPWEN_W<CTL_SPEC> {
        BKPWEN_W::new(self, 8)
    }
}
#[doc = "power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
