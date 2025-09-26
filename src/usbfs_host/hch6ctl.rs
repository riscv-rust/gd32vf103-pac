#[doc = "Register `HCH6CTL` reader"]
pub type R = crate::R<HCH6CTL_SPEC>;
#[doc = "Register `HCH6CTL` writer"]
pub type W = crate::W<HCH6CTL_SPEC>;
#[doc = "Field `MPL` reader - Maximum packet size"]
pub type MPL_R = crate::FieldReader<u16>;
#[doc = "Field `MPL` writer - Maximum packet size"]
pub type MPL_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `EPNUM` writer - Endpoint number"]
pub type EPNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPDIR` reader - Endpoint direction"]
pub type EPDIR_R = crate::BitReader;
#[doc = "Field `EPDIR` writer - Endpoint direction"]
pub type EPDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSD` reader - Low-speed device"]
pub type LSD_R = crate::BitReader;
#[doc = "Field `LSD` writer - Low-speed device"]
pub type LSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EPTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAR` reader - Device address"]
pub type DAR_R = crate::FieldReader;
#[doc = "Field `DAR` writer - Device address"]
pub type DAR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODDFRM` reader - Odd frame"]
pub type ODDFRM_R = crate::BitReader;
#[doc = "Field `ODDFRM` writer - Odd frame"]
pub type ODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDIS` reader - Channel disable"]
pub type CDIS_R = crate::BitReader;
#[doc = "Field `CDIS` writer - Channel disable"]
pub type CDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN` reader - Channel enable"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - Channel enable"]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpl(&self) -> MPL_R {
        MPL_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lsd(&self) -> LSD_R {
        LSD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn cdis(&self) -> CDIS_R {
        CDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    #[must_use]
    pub fn mpl(&mut self) -> MPL_W<HCH6CTL_SPEC> {
        MPL_W::new(self, 0)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<HCH6CTL_SPEC> {
        EPNUM_W::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<HCH6CTL_SPEC> {
        EPDIR_W::new(self, 15)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    #[must_use]
    pub fn lsd(&mut self) -> LSD_W<HCH6CTL_SPEC> {
        LSD_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<HCH6CTL_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<HCH6CTL_SPEC> {
        DAR_W::new(self, 22)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    #[must_use]
    pub fn oddfrm(&mut self) -> ODDFRM_W<HCH6CTL_SPEC> {
        ODDFRM_W::new(self, 29)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    #[must_use]
    pub fn cdis(&mut self) -> CDIS_W<HCH6CTL_SPEC> {
        CDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<HCH6CTL_SPEC> {
        CEN_W::new(self, 31)
    }
}
#[doc = "host channel-6 characteristics register (HCH6CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hch6ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hch6ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCH6CTL_SPEC;
impl crate::RegisterSpec for HCH6CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch6ctl::R`](R) reader structure"]
impl crate::Readable for HCH6CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hch6ctl::W`](W) writer structure"]
impl crate::Writable for HCH6CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCH6CTL to value 0"]
impl crate::Resettable for HCH6CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
