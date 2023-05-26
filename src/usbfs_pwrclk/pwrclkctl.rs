#[doc = "Register `PWRCLKCTL` reader"]
pub struct R(crate::R<PWRCLKCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCLKCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCLKCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCLKCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCLKCTL` writer"]
pub struct W(crate::W<PWRCLKCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCLKCTL_SPEC>;
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
impl From<crate::W<PWRCLKCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCLKCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUCLK` reader - Stop the USB clock"]
pub type SUCLK_R = crate::BitReader<bool>;
#[doc = "Field `SUCLK` writer - Stop the USB clock"]
pub type SUCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCLKCTL_SPEC, bool, O>;
#[doc = "Field `SHCLK` reader - Stop HCLK"]
pub type SHCLK_R = crate::BitReader<bool>;
#[doc = "Field `SHCLK` writer - Stop HCLK"]
pub type SHCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCLKCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Stop the USB clock"]
    #[inline(always)]
    pub fn suclk(&self) -> SUCLK_R {
        SUCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop HCLK"]
    #[inline(always)]
    pub fn shclk(&self) -> SHCLK_R {
        SHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop the USB clock"]
    #[inline(always)]
    #[must_use]
    pub fn suclk(&mut self) -> SUCLK_W<0> {
        SUCLK_W::new(self)
    }
    #[doc = "Bit 1 - Stop HCLK"]
    #[inline(always)]
    #[must_use]
    pub fn shclk(&mut self) -> SHCLK_W<1> {
        SHCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power and clock gating control register (PWRCLKCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrclkctl](index.html) module"]
pub struct PWRCLKCTL_SPEC;
impl crate::RegisterSpec for PWRCLKCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrclkctl::R](R) reader structure"]
impl crate::Readable for PWRCLKCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrclkctl::W](W) writer structure"]
impl crate::Writable for PWRCLKCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCLKCTL to value 0"]
impl crate::Resettable for PWRCLKCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
