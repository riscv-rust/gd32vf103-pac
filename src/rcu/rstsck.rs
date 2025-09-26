#[doc = "Register `RSTSCK` reader"]
pub type R = crate::R<RSTSCK_SPEC>;
#[doc = "Register `RSTSCK` writer"]
pub type W = crate::W<RSTSCK_SPEC>;
#[doc = "Field `IRC40KEN` reader - IRC40K enable"]
pub type IRC40KEN_R = crate::BitReader;
#[doc = "Field `IRC40KEN` writer - IRC40K enable"]
pub type IRC40KEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC40KSTB` reader - IRC40K stabilization"]
pub type IRC40KSTB_R = crate::BitReader;
#[doc = "Field `RSTFC` reader - Reset flag clear"]
pub type RSTFC_R = crate::BitReader;
#[doc = "Field `RSTFC` writer - Reset flag clear"]
pub type RSTFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRSTF` reader - External PIN reset flag"]
pub type EPRSTF_R = crate::BitReader;
#[doc = "Field `PORRSTF` reader - Power reset flag"]
pub type PORRSTF_R = crate::BitReader;
#[doc = "Field `SWRSTF` reader - Software reset flag"]
pub type SWRSTF_R = crate::BitReader;
#[doc = "Field `FWDGTRSTF` reader - Free Watchdog timer reset flag"]
pub type FWDGTRSTF_R = crate::BitReader;
#[doc = "Field `WWDGTRSTF` reader - Window watchdog timer reset flag"]
pub type WWDGTRSTF_R = crate::BitReader;
#[doc = "Field `LPRSTF` reader - Low-power reset flag"]
pub type LPRSTF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IRC40K enable"]
    #[inline(always)]
    pub fn irc40ken(&self) -> IRC40KEN_R {
        IRC40KEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC40K stabilization"]
    #[inline(always)]
    pub fn irc40kstb(&self) -> IRC40KSTB_R {
        IRC40KSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&self) -> RSTFC_R {
        RSTFC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - External PIN reset flag"]
    #[inline(always)]
    pub fn eprstf(&self) -> EPRSTF_R {
        EPRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Power reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&self) -> SWRSTF_R {
        SWRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Free Watchdog timer reset flag"]
    #[inline(always)]
    pub fn fwdgtrstf(&self) -> FWDGTRSTF_R {
        FWDGTRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    pub fn wwdgtrstf(&self) -> WWDGTRSTF_R {
        WWDGTRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lprstf(&self) -> LPRSTF_R {
        LPRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRC40K enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc40ken(&mut self) -> IRC40KEN_W<RSTSCK_SPEC> {
        IRC40KEN_W::new(self, 0)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstfc(&mut self) -> RSTFC_W<RSTSCK_SPEC> {
        RSTFC_W::new(self, 24)
    }
}
#[doc = "Reset source /clock register (RCU_RSTSCK)\n\nYou can [`read`](crate::Reg::read) this register and get [`rstsck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTSCK_SPEC;
impl crate::RegisterSpec for RSTSCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstsck::R`](R) reader structure"]
impl crate::Readable for RSTSCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rstsck::W`](W) writer structure"]
impl crate::Writable for RSTSCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTSCK to value 0x0c00_0000"]
impl crate::Resettable for RSTSCK_SPEC {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
