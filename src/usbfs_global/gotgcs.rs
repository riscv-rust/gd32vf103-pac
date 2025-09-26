#[doc = "Register `GOTGCS` reader"]
pub type R = crate::R<GOTGCS_SPEC>;
#[doc = "Register `GOTGCS` writer"]
pub type W = crate::W<GOTGCS_SPEC>;
#[doc = "Field `SRPS` reader - SRP success"]
pub type SRPS_R = crate::BitReader;
#[doc = "Field `SRPREQ` reader - SRP request"]
pub type SRPREQ_R = crate::BitReader;
#[doc = "Field `SRPREQ` writer - SRP request"]
pub type SRPREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPS` reader - Host success"]
pub type HNPS_R = crate::BitReader;
#[doc = "Field `HNPREQ` reader - HNP request"]
pub type HNPREQ_R = crate::BitReader;
#[doc = "Field `HNPREQ` writer - HNP request"]
pub type HNPREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HHNPEN` reader - Host HNP enable"]
pub type HHNPEN_R = crate::BitReader;
#[doc = "Field `HHNPEN` writer - Host HNP enable"]
pub type HHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHNPEN` reader - Device HNP enabled"]
pub type DHNPEN_R = crate::BitReader;
#[doc = "Field `DHNPEN` writer - Device HNP enabled"]
pub type DHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDPS` reader - ID pin status"]
pub type IDPS_R = crate::BitReader;
#[doc = "Field `DI` reader - Debounce interval"]
pub type DI_R = crate::BitReader;
#[doc = "Field `ASV` reader - A-session valid"]
pub type ASV_R = crate::BitReader;
#[doc = "Field `BSV` reader - B-session valid"]
pub type BSV_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SRP success"]
    #[inline(always)]
    pub fn srps(&self) -> SRPS_R {
        SRPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRP request"]
    #[inline(always)]
    pub fn srpreq(&self) -> SRPREQ_R {
        SRPREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Host success"]
    #[inline(always)]
    pub fn hnps(&self) -> HNPS_R {
        HNPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host HNP enable"]
    #[inline(always)]
    pub fn hhnpen(&self) -> HHNPEN_R {
        HHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ID pin status"]
    #[inline(always)]
    pub fn idps(&self) -> IDPS_R {
        IDPS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Debounce interval"]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsv(&self) -> BSV_R {
        BSV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SRP request"]
    #[inline(always)]
    #[must_use]
    pub fn srpreq(&mut self) -> SRPREQ_W<GOTGCS_SPEC> {
        SRPREQ_W::new(self, 1)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    #[must_use]
    pub fn hnpreq(&mut self) -> HNPREQ_W<GOTGCS_SPEC> {
        HNPREQ_W::new(self, 9)
    }
    #[doc = "Bit 10 - Host HNP enable"]
    #[inline(always)]
    #[must_use]
    pub fn hhnpen(&mut self) -> HHNPEN_W<GOTGCS_SPEC> {
        HHNPEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dhnpen(&mut self) -> DHNPEN_W<GOTGCS_SPEC> {
        DHNPEN_W::new(self, 11)
    }
}
#[doc = "Global OTG control and status register (USBFS_GOTGCS)\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGCS_SPEC;
impl crate::RegisterSpec for GOTGCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgcs::R`](R) reader structure"]
impl crate::Readable for GOTGCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gotgcs::W`](W) writer structure"]
impl crate::Writable for GOTGCS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGCS to value 0x0800"]
impl crate::Resettable for GOTGCS_SPEC {
    const RESET_VALUE: u32 = 0x0800;
}
