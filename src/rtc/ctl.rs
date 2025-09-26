#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `SCIF` reader - Sencond interrupt flag"]
pub type SCIF_R = crate::BitReader;
#[doc = "Field `SCIF` writer - Sencond interrupt flag"]
pub type SCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRMIF` reader - Alarm interrupt flag"]
pub type ALRMIF_R = crate::BitReader;
#[doc = "Field `ALRMIF` writer - Alarm interrupt flag"]
pub type ALRMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVIF` reader - Overflow interrupt flag"]
pub type OVIF_R = crate::BitReader;
#[doc = "Field `OVIF` writer - Overflow interrupt flag"]
pub type OVIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSYNF` reader - Registers synchronized flag"]
pub type RSYNF_R = crate::BitReader;
#[doc = "Field `RSYNF` writer - Registers synchronized flag"]
pub type RSYNF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMF` reader - Configuration mode flag"]
pub type CMF_R = crate::BitReader;
#[doc = "Field `CMF` writer - Configuration mode flag"]
pub type CMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LWOFF` reader - Last write operation finished flag"]
pub type LWOFF_R = crate::BitReader;
#[doc = "Field `LWOFF` writer - Last write operation finished flag"]
pub type LWOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sencond interrupt flag"]
    #[inline(always)]
    pub fn scif(&self) -> SCIF_R {
        SCIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt flag"]
    #[inline(always)]
    pub fn alrmif(&self) -> ALRMIF_R {
        ALRMIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovif(&self) -> OVIF_R {
        OVIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    pub fn rsynf(&self) -> RSYNF_R {
        RSYNF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration mode flag"]
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Last write operation finished flag"]
    #[inline(always)]
    pub fn lwoff(&self) -> LWOFF_R {
        LWOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sencond interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn scif(&mut self) -> SCIF_W<CTL_SPEC> {
        SCIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrmif(&mut self) -> ALRMIF_W<CTL_SPEC> {
        ALRMIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovif(&mut self) -> OVIF_W<CTL_SPEC> {
        OVIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsynf(&mut self) -> RSYNF_W<CTL_SPEC> {
        RSYNF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configuration mode flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmf(&mut self) -> CMF_W<CTL_SPEC> {
        CMF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Last write operation finished flag"]
    #[inline(always)]
    #[must_use]
    pub fn lwoff(&mut self) -> LWOFF_W<CTL_SPEC> {
        LWOFF_W::new(self, 5)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0x20"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
