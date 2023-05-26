#[doc = "Register `SWT` writer"]
pub struct W(crate::W<SWT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWT_SPEC>;
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
impl From<crate::W<SWT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWTR0` writer - DAC0 software trigger"]
pub type SWTR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWT_SPEC, bool, O>;
#[doc = "Field `SWTR1` writer - DAC1 software trigger"]
pub type SWTR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWT_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DAC0 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtr0(&mut self) -> SWTR0_W<0> {
        SWTR0_W::new(self)
    }
    #[doc = "Bit 1 - DAC1 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtr1(&mut self) -> SWTR1_W<1> {
        SWTR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "software trigger register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swt](index.html) module"]
pub struct SWT_SPEC;
impl crate::RegisterSpec for SWT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swt::W](W) writer structure"]
impl crate::Writable for SWT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWT to value 0"]
impl crate::Resettable for SWT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
