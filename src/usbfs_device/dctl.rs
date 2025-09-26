#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DCTL_SPEC>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DCTL_SPEC>;
#[doc = "Field `RWKUP` reader - Remote wakeup"]
pub type RWKUP_R = crate::BitReader;
#[doc = "Field `RWKUP` writer - Remote wakeup"]
pub type RWKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD` reader - Soft disconnect"]
pub type SD_R = crate::BitReader;
#[doc = "Field `SD` writer - Soft disconnect"]
pub type SD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINS` reader - Global IN NAK status"]
pub type GINS_R = crate::BitReader;
#[doc = "Field `GONS` reader - Global OUT NAK status"]
pub type GONS_R = crate::BitReader;
#[doc = "Field `SGINAK` writer - Set global IN NAK"]
pub type SGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGINAK` writer - Clear global IN NAK"]
pub type CGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGONAK` writer - Set global OUT NAK"]
pub type SGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGONAK` writer - Clear global OUT NAK"]
pub type CGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POIF` reader - Power-on initialization flag"]
pub type POIF_R = crate::BitReader;
#[doc = "Field `POIF` writer - Power-on initialization flag"]
pub type POIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote wakeup"]
    #[inline(always)]
    pub fn rwkup(&self) -> RWKUP_R {
        RWKUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sd(&self) -> SD_R {
        SD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn gins(&self) -> GINS_R {
        GINS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn gons(&self) -> GONS_R {
        GONS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Power-on initialization flag"]
    #[inline(always)]
    pub fn poif(&self) -> POIF_R {
        POIF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn rwkup(&mut self) -> RWKUP_W<DCTL_SPEC> {
        RWKUP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn sd(&mut self) -> SD_W<DCTL_SPEC> {
        SD_W::new(self, 1)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sginak(&mut self) -> SGINAK_W<DCTL_SPEC> {
        SGINAK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cginak(&mut self) -> CGINAK_W<DCTL_SPEC> {
        CGINAK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgonak(&mut self) -> SGONAK_W<DCTL_SPEC> {
        SGONAK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgonak(&mut self) -> CGONAK_W<DCTL_SPEC> {
        CGONAK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Power-on initialization flag"]
    #[inline(always)]
    #[must_use]
    pub fn poif(&mut self) -> POIF_W<DCTL_SPEC> {
        POIF_W::new(self, 11)
    }
}
#[doc = "device control register (DCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTL to value 0"]
impl crate::Resettable for DCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
