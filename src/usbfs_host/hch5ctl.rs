#[doc = "Register `HCH5CTL` reader"]
pub type R = crate::R<HCH5CTL_SPEC>;
#[doc = "Register `HCH5CTL` writer"]
pub type W = crate::W<HCH5CTL_SPEC>;
#[doc = "Field `MPL` reader - Maximum packet size"]
pub type MPL_R = crate::FieldReader<u16>;
#[doc = "Field `MPL` writer - Maximum packet size"]
pub type MPL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `EPNUM` writer - Endpoint number"]
pub type EPNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EPDIR` reader - Endpoint direction"]
pub type EPDIR_R = crate::BitReader;
#[doc = "Field `EPDIR` writer - Endpoint direction"]
pub type EPDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSD` reader - Low-speed device"]
pub type LSD_R = crate::BitReader;
#[doc = "Field `LSD` writer - Low-speed device"]
pub type LSD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EPTYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DAR` reader - Device address"]
pub type DAR_R = crate::FieldReader;
#[doc = "Field `DAR` writer - Device address"]
pub type DAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `ODDFRM` reader - Odd frame"]
pub type ODDFRM_R = crate::BitReader;
#[doc = "Field `ODDFRM` writer - Odd frame"]
pub type ODDFRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDIS` reader - Channel disable"]
pub type CDIS_R = crate::BitReader;
#[doc = "Field `CDIS` writer - Channel disable"]
pub type CDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CEN` reader - Channel enable"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - Channel enable"]
pub type CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn mpl(&mut self) -> MPL_W<HCH5CTL_SPEC, 0> {
        MPL_W::new(self)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<HCH5CTL_SPEC, 11> {
        EPNUM_W::new(self)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<HCH5CTL_SPEC, 15> {
        EPDIR_W::new(self)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    #[must_use]
    pub fn lsd(&mut self) -> LSD_W<HCH5CTL_SPEC, 17> {
        LSD_W::new(self)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<HCH5CTL_SPEC, 18> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<HCH5CTL_SPEC, 22> {
        DAR_W::new(self)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    #[must_use]
    pub fn oddfrm(&mut self) -> ODDFRM_W<HCH5CTL_SPEC, 29> {
        ODDFRM_W::new(self)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    #[must_use]
    pub fn cdis(&mut self) -> CDIS_W<HCH5CTL_SPEC, 30> {
        CDIS_W::new(self)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<HCH5CTL_SPEC, 31> {
        CEN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "host channel-5 characteristics register (HCH5CTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hch5ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hch5ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCH5CTL_SPEC;
impl crate::RegisterSpec for HCH5CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hch5ctl::R`](R) reader structure"]
impl crate::Readable for HCH5CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hch5ctl::W`](W) writer structure"]
impl crate::Writable for HCH5CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCH5CTL to value 0"]
impl crate::Resettable for HCH5CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
