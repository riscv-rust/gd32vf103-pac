#[doc = "Register `DCTL` reader"]
pub struct R(crate::R<DCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCTL` writer"]
pub struct W(crate::W<DCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTL_SPEC>;
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
impl From<crate::W<DCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWKUP` reader - Remote wakeup"]
pub type RWKUP_R = crate::BitReader<bool>;
#[doc = "Field `RWKUP` writer - Remote wakeup"]
pub type RWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `SD` reader - Soft disconnect"]
pub type SD_R = crate::BitReader<bool>;
#[doc = "Field `SD` writer - Soft disconnect"]
pub type SD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `GINS` reader - Global IN NAK status"]
pub type GINS_R = crate::BitReader<bool>;
#[doc = "Field `GONS` reader - Global OUT NAK status"]
pub type GONS_R = crate::BitReader<bool>;
#[doc = "Field `SGINAK` writer - Set global IN NAK"]
pub type SGINAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `CGINAK` writer - Clear global IN NAK"]
pub type CGINAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `SGONAK` writer - Set global OUT NAK"]
pub type SGONAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `CGONAK` writer - Clear global OUT NAK"]
pub type CGONAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `POIF` reader - Power-on initialization flag"]
pub type POIF_R = crate::BitReader<bool>;
#[doc = "Field `POIF` writer - Power-on initialization flag"]
pub type POIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
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
    pub fn rwkup(&mut self) -> RWKUP_W<0> {
        RWKUP_W::new(self)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn sd(&mut self) -> SD_W<1> {
        SD_W::new(self)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sginak(&mut self) -> SGINAK_W<7> {
        SGINAK_W::new(self)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cginak(&mut self) -> CGINAK_W<8> {
        CGINAK_W::new(self)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgonak(&mut self) -> SGONAK_W<9> {
        SGONAK_W::new(self)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgonak(&mut self) -> CGONAK_W<10> {
        CGONAK_W::new(self)
    }
    #[doc = "Bit 11 - Power-on initialization flag"]
    #[inline(always)]
    #[must_use]
    pub fn poif(&mut self) -> POIF_W<11> {
        POIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device control register (DCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctl](index.html) module"]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dctl::R](R) reader structure"]
impl crate::Readable for DCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dctl::W](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCTL to value 0"]
impl crate::Resettable for DCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
