#[doc = "Register `RSTSCK` reader"]
pub struct R(crate::R<RSTSCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTSCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTSCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTSCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTSCK` writer"]
pub struct W(crate::W<RSTSCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTSCK_SPEC>;
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
impl From<crate::W<RSTSCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTSCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRC40KEN` reader - IRC40K enable"]
pub type IRC40KEN_R = crate::BitReader<bool>;
#[doc = "Field `IRC40KEN` writer - IRC40K enable"]
pub type IRC40KEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, O>;
#[doc = "Field `IRC40KSTB` reader - IRC40K stabilization"]
pub type IRC40KSTB_R = crate::BitReader<bool>;
#[doc = "Field `RSTFC` reader - Reset flag clear"]
pub type RSTFC_R = crate::BitReader<bool>;
#[doc = "Field `RSTFC` writer - Reset flag clear"]
pub type RSTFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTSCK_SPEC, bool, O>;
#[doc = "Field `EPRSTF` reader - External PIN reset flag"]
pub type EPRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PORRSTF` reader - Power reset flag"]
pub type PORRSTF_R = crate::BitReader<bool>;
#[doc = "Field `SWRSTF` reader - Software reset flag"]
pub type SWRSTF_R = crate::BitReader<bool>;
#[doc = "Field `FWDGTRSTF` reader - Free Watchdog timer reset flag"]
pub type FWDGTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `WWDGTRSTF` reader - Window watchdog timer reset flag"]
pub type WWDGTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `LPRSTF` reader - Low-power reset flag"]
pub type LPRSTF_R = crate::BitReader<bool>;
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
    pub fn irc40ken(&mut self) -> IRC40KEN_W<0> {
        IRC40KEN_W::new(self)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstfc(&mut self) -> RSTFC_W<24> {
        RSTFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset source /clock register (RCU_RSTSCK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstsck](index.html) module"]
pub struct RSTSCK_SPEC;
impl crate::RegisterSpec for RSTSCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstsck::R](R) reader structure"]
impl crate::Readable for RSTSCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstsck::W](W) writer structure"]
impl crate::Writable for RSTSCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTSCK to value 0x0c00_0000"]
impl crate::Resettable for RSTSCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0000;
}
